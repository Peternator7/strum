use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::helpers::{HasStrumVariantProperties, HasTypeProperties};

pub fn from_string_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => panic!("FromString only works on Enums"),
    };

    let type_properties = ast.get_type_properties()?;

    let mut has_default = false;
    let mut default =
        quote! { _ => ::std::result::Result::Err(::strum::ParseError::VariantNotFound) };
    let mut arms = Vec::new();
    for variant in variants {
        use syn::Fields::*;
        let ident = &variant.ident;
        let variant_properties = variant.get_variant_properties()?;

        if variant_properties.is_disabled {
            continue;
        }

        if variant_properties.default {
            if has_default {
                panic!("Can't have multiple default variants");
            }

            if let Unnamed(fields) = &variant.fields {
                if fields.unnamed.len() != 1 {
                    panic!("Default only works on unit structs with a single String parameter");
                }

                default = quote! {
                    default => ::std::result::Result::Ok(#name::#ident (default.into()))
                };
            } else {
                panic!("Default only works on unit structs with a single String parameter");
            }

            has_default = true;
            continue;
        }

        // If we don't have any custom variants, add the default serialized name.
        let attrs = variant_properties.get_serializations(type_properties.case_style);

        let params = match &variant.fields {
            Unit => quote! {},
            Unnamed(fields) => {
                let defaults =
                    ::std::iter::repeat(quote!(Default::default())).take(fields.unnamed.len());
                quote! { (#(#defaults),*) }
            }
            Named(fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                quote! { {#(#fields: Default::default()),*} }
            }
        };

        arms.push(quote! { #(#attrs)|* => ::std::result::Result::Ok(#name::#ident #params) });
    }

    arms.push(default);

    Ok(quote! {
        impl #impl_generics ::std::str::FromStr for #name #ty_generics #where_clause {
            type Err = ::strum::ParseError;
            fn from_str(s: &str) -> ::std::result::Result< #name #ty_generics , Self::Err> {
                match s {
                    #(#arms),*
                }
            }
        }
    })
}
