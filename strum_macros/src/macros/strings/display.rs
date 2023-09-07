use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, Data, DeriveInput, Fields, Token};

use crate::helpers::{non_enum_error, HasStrumVariantProperties, HasTypeProperties};

pub fn display_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let type_properties = ast.get_type_properties()?;

    let mut arms = Vec::new();
    for variant in variants {
        let ident = &variant.ident;
        let variant_properties = variant.get_variant_properties()?;

        if variant_properties.disabled.is_some() {
            continue;
        }

        // Look at all the serialize attributes.
        let output = variant_properties.get_preferred_name(type_properties.case_style);

        let params = match variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(..) => quote! { (..) },
            Fields::Named(ref field_names) => {
                let names: Punctuated<&Ident, Token!(,)> = field_names
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap())
                    .collect();
                quote! { {#names} }
            }
        };

        if variant_properties.to_string.is_none() && variant_properties.default.is_some() {
            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    arms.push(quote! { #name::#ident(ref s) => f.pad(s) });
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "Default only works on newtype structs with a single String field",
                    ))
                }
            }
        } else {
            let arm = match variant.fields {
                Fields::Named(_) => quote! {
                    #[allow(unused_variables)]
                    #name::#ident #params => f.pad(&format!(#output))
                },
                _ => quote! { #name::#ident #params => f.pad(#output) },
            };
            arms.push(arm);
        }
    }

    if arms.len() < variants.len() {
        arms.push(quote! { _ => panic!("fmt() called on disabled variant.") });
    }

    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error> {
                match *self {
                    #(#arms),*
                }
            }
        }
    })
}
