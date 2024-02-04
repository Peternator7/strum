use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::helpers::{
    non_enum_error, non_single_field_variant_error, HasStrumVariantProperties, HasTypeProperties,
};

fn get_arms(ast: &DeriveInput) -> syn::Result<Vec<TokenStream>> {
    let name = &ast.ident;
    let mut arms = Vec::new();
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let type_properties = ast.get_type_properties()?;

    for variant in variants {
        let ident = &variant.ident;
        let variant_properties = variant.get_variant_properties()?;

        if variant_properties.disabled.is_some() {
            continue;
        }

        let arm = if variant_properties.transparent.is_some() {
            let arm_end = match &variant.fields {
                Fields::Unnamed(f) if f.unnamed.len() == 1 => {
                    quote! { (ref v) => ::core::convert::AsRef::<str>::as_ref(v) }
                }
                Fields::Named(f) if f.named.len() == 1 => {
                    let ident = f.named.last().unwrap().ident.as_ref().unwrap();
                    quote! { {ref #ident} => ::core::convert::AsRef::<str>::as_ref(#ident) }
                }
                _ => return Err(non_single_field_variant_error("transparent")),
            };

            quote! { #name::#ident #arm_end }
        } else {
            // Look at all the serialize attributes.
            // Use `to_string` attribute (not `as_ref_str` or something) to keep things consistent
            // (i.e. always `enum.as_ref().to_string() == enum.to_string()`).
            let output = variant_properties.get_preferred_name(type_properties.case_style);
            let params = match variant.fields {
                Fields::Unit => quote! {},
                Fields::Unnamed(..) => quote! { (..) },
                Fields::Named(..) => quote! { {..} },
            };

            quote! { #name::#ident #params => #output }
        };

        arms.push(arm);
    }

    if arms.len() < variants.len() {
        arms.push(quote! {
            _ => panic!(
                "AsRef::<str>::as_ref() or AsStaticRef::<str>::as_static() \
                 called on disabled variant.",
            )
        });
    }

    Ok(arms)
}

pub fn as_ref_str_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let arms = get_arms(ast)?;
    Ok(quote! {
        impl #impl_generics ::core::convert::AsRef<str> for #name #ty_generics #where_clause {
            fn as_ref(&self) -> &str {
                match *self {
                    #(#arms),*
                }
            }
        }
    })
}
