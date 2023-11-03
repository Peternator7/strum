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

    let mut phf_exact_match_arms = Vec::new();
    let mut standard_match_arms = Vec::new();
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

        let is_ascii_case_insensitive = variant_properties
            .ascii_case_insensitive
            .unwrap_or(type_properties.ascii_case_insensitive);

        // If we don't have any custom variants, add the default serialized name.
        for serialization in variant_properties.get_serializations(type_properties.case_style) {
            if type_properties.use_phf {
                phf_exact_match_arms.push(quote! { #serialization => #name::#ident #params, });

                if is_ascii_case_insensitive {
                    // Store the lowercase and UPPERCASE variants in the phf map to capture
                    let ser_string = serialization.value();

                    let lower =
                        syn::LitStr::new(&ser_string.to_ascii_lowercase(), serialization.span());
                    let upper =
                        syn::LitStr::new(&ser_string.to_ascii_uppercase(), serialization.span());
                    phf_exact_match_arms.push(quote! { #lower => #name::#ident #params, });
                    phf_exact_match_arms.push(quote! { #upper => #name::#ident #params, });
                    standard_match_arms.push(quote! { s if s.eq_ignore_ascii_case(#serialization) => #name::#ident #params, });
                }
            } else {
                standard_match_arms.push(if !is_ascii_case_insensitive {
                    quote! { #serialization => #name::#ident #params, }
                } else {
                    quote! { s if s.eq_ignore_ascii_case(#serialization) => #name::#ident #params, }
                });
            }
        }
    }

    let phf_body = if phf_exact_match_arms.is_empty() {
        quote!()
    } else {
        quote! {
            use #strum_module_path::_private_phf_reexport_for_macro_if_phf_feature as phf;
            static PHF: phf::Map<&'static str, #name> = phf::phf_map! {
                #(#phf_exact_match_arms)*
            };
            if let Some(value) = PHF.get(s).cloned() {
                return ::core::result::Result::Ok(value);
            }
        }
    };

    let try_from_body = |ty: TokenStream,
                         standard_match_body: &TokenStream,
                         phf_body: &TokenStream| {
        quote! {
                #[allow(clippy::use_self)]
                impl #impl_generics ::core::convert::TryFrom<#ty> for #name #ty_generics #where_clause {
                    type Error = #strum_module_path::ParseError;
                    fn try_from(s: #ty) -> ::core::result::Result< #name #ty_generics , <Self as ::core::convert::TryFrom<#ty>>::Error> {
                        #phf_body
                        #standard_match_body
                }
            }
        }
    };
    #[cfg(feature = "std")]
    let try_from_string = {
        // Takes the match body and the phf body and returns

        // If it has a default build the match body and phf body
        // Else just pass to FromStr
        if default_kw.is_some() {
            if standard_match_arms.is_empty() {
                try_from_body(quote! {::std::string::String}, &default, &quote!())
            } else {
                let standard_match_body = quote! {
                    ::core::result::Result::Ok(match s.as_str() {
                        #(#standard_match_arms)*
                        _ => return #default,
                    })
                };
                let phf_body = if phf_exact_match_arms.is_empty() {
                    quote!()
                } else {
                    quote! {
                        use #strum_module_path::_private_phf_reexport_for_macro_if_phf_feature as phf;
                        static PHF: phf::Map<&'static str, #name> = phf::phf_map! {
                            #(#phf_exact_match_arms)*
                        };
                        if let Some(value) = PHF.get(s.as_str()).cloned() {
                            return ::core::result::Result::Ok(value);
                        }
                    }
                };
                try_from_body(
                    quote! {::std::string::String},
                    &standard_match_body,
                    &phf_body,
                )
            }
        } else {
            try_from_body(
                quote! {::std::string::String},
                &quote! {
                    ::core::str::FromStr::from_str(s.as_str())
                },
                &quote!(),
            )
        }
    };
    #[cfg(not(feature = "std"))]
    let try_from_string = quote! {};

    let standard_match_body = if standard_match_arms.is_empty() {
        default
    } else {
        quote! {
            ::core::result::Result::Ok(match s {
                #(#standard_match_arms)*
                _ => return #default,
            })
        }
    };
    let try_from_str = try_from_body(quote! {&str}, &standard_match_body, &phf_body);
    let from_str = quote! {
        #[allow(clippy::use_self)]
        impl #impl_generics ::core::str::FromStr for #name #ty_generics #where_clause {
            type Err = #strum_module_path::ParseError;
            fn from_str(s: &str) -> ::core::result::Result< #name #ty_generics , <Self as ::core::str::FromStr>::Err> {
                ::core::convert::TryFrom::try_from(s)
            }
        }
    };

    Ok(quote! {
        #try_from_str
        #from_str
        #try_from_string
    })
}
