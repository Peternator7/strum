use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::helpers::{
    non_enum_error, occurrence_error, HasStrumVariantProperties, HasTypeProperties,
};

pub fn from_string_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let type_properties = ast.get_type_properties()?;
    let strum_module_path = type_properties.crate_module_path();

    let mut default_kw = None;
    let mut default =
        quote! { ::core::result::Result::Err(#strum_module_path::ParseError::VariantNotFound) };
    let mut arms = Vec::new();
    for variant in variants {
        let ident = &variant.ident;
        let variant_properties = variant.get_variant_properties()?;

        if variant_properties.disabled.is_some() {
            continue;
        }

        if let Some(kw) = variant_properties.default {
            if let Some(fst_kw) = default_kw {
                return Err(occurrence_error(fst_kw, kw, "default"));
            }

            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {}
                _ => {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "Default only works on newtype structs with a single String field",
                    ))
                }
            }

            default_kw = Some(kw);
            default = quote! {
                ::core::result::Result::Ok(#name::#ident(s.into()))
            };
            continue;
        }

        let is_ascii_case_insensitive = match (
            type_properties.use_phf,
            type_properties.ascii_case_insensitive,
            variant_properties.ascii_case_insensitive,
        ) {
            (false, _, Some(variant_case)) => variant_case,
            (false, type_case, None) => type_case,
            (true, false, None | Some(false)) => false,
            (true, true, None | Some(true)) => true,
            (true, false, Some(true)) | (true, true, Some(false)) => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "Cannot use per-variant case insensitive parsing in combination with `phf`",
                ))
            }
        };
        // If we don't have any custom variants, add the default serialized name.
        let attrs = variant_properties
            .get_serializations(type_properties.case_style)
            .into_iter()
            .map(|serialization| {
                if is_ascii_case_insensitive {
                    if type_properties.use_phf {
                        // In that case we'll store the lowercase values in phf, and lowercase at runtime
                        // before searching
                        let mut ser_string = serialization.value();
                        ser_string.make_ascii_lowercase();
                        let serialization = syn::LitStr::new(&ser_string, serialization.span());
                        quote! { #serialization }
                    } else {
                        quote! { s if s.eq_ignore_ascii_case(#serialization) }
                    }
                } else {
                    quote! { #serialization }
                }
            });

        let params = match &variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(fields) => {
                let defaults =
                    ::core::iter::repeat(quote!(Default::default())).take(fields.unnamed.len());
                quote! { (#(#defaults),*) }
            }
            Fields::Named(fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                quote! { {#(#fields: Default::default()),*} }
            }
        };

        arms.push(quote! { #(#attrs => #name::#ident #params,)* });
    }

    let body = if type_properties.use_phf {
        let maybe_to_lowercase = if type_properties.ascii_case_insensitive {
            // This will allocate but I'm afraid we don't have anything better ATM :(
            quote! {
                let s = &s.to_ascii_lowercase();
            }
        } else {
            quote!()
        };
        quote! {
            use #strum_module_path::_private_phf_reexport_for_macro_if_phf_feature as phf;
            static PHF: phf::Map<&'static str, #name> = phf::phf_map! {
                #(#arms)*
            };
            #maybe_to_lowercase
            PHF.get(s).cloned().map_or_else(|| #default, ::core::result::Result::Ok)
        }
    } else {
        quote! {
            ::core::result::Result::Ok(match s {
                #(#arms)*
                _ => return #default,
            })
        }
    };

    let from_str = quote! {
        #[allow(clippy::use_self)]
        impl #impl_generics ::core::str::FromStr for #name #ty_generics #where_clause {
            type Err = #strum_module_path::ParseError;
            fn from_str(s: &str) -> ::core::result::Result< #name #ty_generics , <Self as ::core::str::FromStr>::Err> {
                #body
            }
        }
    };

    let try_from_str = try_from_str(
        name,
        &impl_generics,
        &ty_generics,
        where_clause,
        &strum_module_path,
    );

    Ok(quote! {
        #from_str
        #try_from_str
    })
}

#[rustversion::before(1.34)]
fn try_from_str(
    _name: &proc_macro2::Ident,
    _impl_generics: &syn::ImplGenerics,
    _ty_generics: &syn::TypeGenerics,
    _where_clause: Option<&syn::WhereClause>,
    _strum_module_path: &syn::Path,
) -> TokenStream {
    Default::default()
}

#[rustversion::since(1.34)]
fn try_from_str(
    name: &proc_macro2::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    strum_module_path: &syn::Path,
) -> TokenStream {
    quote! {
        #[allow(clippy::use_self)]
        impl #impl_generics ::core::convert::TryFrom<&str> for #name #ty_generics #where_clause {
            type Error = #strum_module_path::ParseError;
            fn try_from(s: &str) -> ::core::result::Result< #name #ty_generics , <Self as ::core::convert::TryFrom<&str>>::Error> {
                ::core::str::FromStr::from_str(s)
            }
        }
    }
}
