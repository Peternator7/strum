use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit};

use crate::helpers::{non_enum_error, HasStrumVariantProperties, HasTypeProperties};

enum PropertyType {
    String = 0,
    Integer = 1,
    Bool = 2,
}

const PROPERTY_TYPES: usize = 3;

pub fn enum_properties_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };
    let type_properties = ast.get_type_properties()?;
    let strum_module_path = type_properties.crate_module_path();

    let mut built_arms: [Vec<TokenStream>; 3] = Default::default();

    for variant in variants {
        let ident = &variant.ident;
        let variant_properties = variant.get_variant_properties()?;
        let mut arms: [Vec<_>; 3] = Default::default();
        // But you can disable the messages.
        if variant_properties.disabled.is_some() {
            continue;
        }

        let params = match variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(..) => quote! { (..) },
            Fields::Named(..) => quote! { {..} },
        };

        for (key, value) in variant_properties.props {
            let property_type = match value {
                Lit::Str(..) => PropertyType::String,
                Lit::Bool(..) => PropertyType::Bool,
                Lit::Int(..) => PropertyType::Integer,
                _ => todo!("TODO"),
            };

            arms[property_type as usize]
                .push(quote! { #key => ::core::option::Option::Some( #value )});
        }

        for i in 0..PROPERTY_TYPES {
            arms[i].push(quote! { _ => ::core::option::Option::None });
            let arms_as_string = &arms[i];
            built_arms[i].push(quote! {
                &#name::#ident #params => {
                    match prop {
                        #(#arms_as_string),*
                    }
                }
            });
        }
    }

    for arms in built_arms.iter_mut() {
        if arms.len() < variants.len() {
            arms.push(quote! { _ => ::core::option::Option::None });
        }
    }

    let (built_string_arms, built_int_arms, built_bool_arms) = (
        &built_arms[PropertyType::String as usize],
        &built_arms[PropertyType::Integer as usize],
        &built_arms[PropertyType::Bool as usize],
    );

    Ok(quote! {
        impl #impl_generics #strum_module_path::EnumProperty for #name #ty_generics #where_clause {
            #[inline]
            fn get_str(&self, prop: &str) -> ::core::option::Option<&'static str> {
                match self {
                    #(#built_string_arms),*
                }
            }

            #[inline]
            fn get_int(&self, prop: &str) -> ::core::option::Option<i64> {
                match self {
                    #(#built_int_arms),*
                }
            }

            #[inline]
            fn get_bool(&self, prop: &str) -> ::core::option::Option<bool> {
                match self {
                    #(#built_bool_arms),*
                }
            }

        }
    })
}
