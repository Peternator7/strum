use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput};

use crate::helpers::{non_enum_error, HasStrumVariantProperties};

pub fn enum_const_index_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let vis = &ast.vis;

    if gen.lifetimes().count() > 0 {
        return Err(syn::Error::new(
            Span::call_site(),
            "This macro doesn't support enums with lifetimes. \
             The resulting enums would be unbounded.",
        ));
    }

    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let mut arms = Vec::new();
    let mut idx = 0usize;
    for variant in variants {
        use syn::Fields::*;

        if variant.get_variant_properties()?.disabled.is_some() {
            continue;
        }

        let ident = &variant.ident;
        let params = match &variant.fields {
            Unit => quote! {},
            Unnamed(fields) => {
                let defaults = ::std::iter::repeat(quote!(::core::default::Default::default()))
                    .take(fields.unnamed.len());
                quote! { (#(#defaults),*) }
            }
            Named(fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                quote! { {#(#fields: ::core::default::Default::default()),*} }
            }
        };

        if let Some((_eq,expr)) = &variant.discriminant {
            if let syn::Expr::Lit(expr_lit) = expr {
                if let syn::Lit::Int(int_lit) = &expr_lit.lit {
                    if let Ok(v) = int_lit.base10_parse() {
                        idx = v;
                    } else {
                        panic!()
                    }
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        }

        arms.push(quote! {#idx => ::core::option::Option::Some(#name::#ident #params)});
        idx += 1;
    }

    arms.push(quote! { _ => ::core::option::Option::None });

    Ok(quote! {
        impl #name #gen {
            #vis const fn const_get(idx: usize) -> Option<#name #gen> {
                match idx {
                    #(#arms),*
                }
            }
        }
    })
}
