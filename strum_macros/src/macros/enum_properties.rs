use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::helpers::{non_enum_error, HasStrumVariantProperties};

pub fn enum_properties_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let mut arms = Vec::new();
    for variant in variants {
        let ident = &variant.ident;
        let variant_properties = variant.get_variant_properties()?;
        let mut string_arms = Vec::new();
        let mut bool_arms = Vec::new();
        let mut num_arms = Vec::new();
        // But you can disable the messages.
        if variant_properties.disabled.is_some() {
            continue;
        }

        use syn::Fields::*;
        let params = match variant.fields {
            Unit => quote! {},
            Unnamed(..) => quote! { (..) },
            Named(..) => quote! { {..} },
        };

        for (key, value) in variant_properties.string_props {
            string_arms.push(quote! { #key => ::std::option::Option::Some( #value )})
        }

        string_arms.push(quote! { _ => ::std::option::Option::None });
        bool_arms.push(quote! { _ => ::std::option::Option::None });
        num_arms.push(quote! { _ => ::std::option::Option::None });

        arms.push(quote! {
            &#name::#ident #params => {
                match prop {
                    #(#string_arms),*
                }
            }
        });
    }

    if arms.len() < variants.len() {
        arms.push(quote! { _ => ::std::option::Option::None });
    }

    Ok(quote! {
        impl #impl_generics ::strum::EnumProperty for #name #ty_generics #where_clause {
            fn get_str(&self, prop: &str) -> ::std::option::Option<&'static str> {
                match self {
                    #(#arms),*
                }
            }
        }
    })
}
