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
    let mut default = quote! { _ => ::core::result::Result::Err(#strum_module_path::ParseError::VariantNotFound) };
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
                default => ::core::result::Result::Ok(#name::#ident(default.into()))
            };
            continue;
        }

        let is_ascii_case_insensitive = variant_properties
            .ascii_case_insensitive
            .unwrap_or(type_properties.ascii_case_insensitive);
        // If we don't have any custom variants, add the default serialized name.
        let attrs = variant_properties
            .get_serializations(type_properties.case_style)
            .into_iter()
            .map(|serialization| {
                if is_ascii_case_insensitive {
                    quote! { s if s.eq_ignore_ascii_case(#serialization) }
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

        arms.push(quote! { #(#attrs => ::core::result::Result::Ok(#name::#ident #params)),* });
    }

    arms.push(default);

    let from_str = quote! {
        #[allow(clippy::use_self)]
        impl #impl_generics ::core::str::FromStr for #name #ty_generics #where_clause {
            type Err = #strum_module_path::ParseError;
            fn from_str(s: &str) -> ::core::result::Result< #name #ty_generics , <Self as ::core::str::FromStr>::Err> {
                match s {
                    #(#arms),*
                }
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
