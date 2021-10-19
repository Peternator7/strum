use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, PathArguments, Type, TypeParen, Ident};

use crate::helpers::{non_enum_error, HasStrumVariantProperties};

pub fn enum_index_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let vis = &ast.vis;
    let attrs = &ast.attrs;

    let mod_name = quote::format_ident!("{}Index",name);

    let mut index_type: Type = syn::parse("usize".parse().unwrap()).unwrap();
    for attr in attrs {
        let path = &attr.path;
        let tokens = &attr.tokens;
        if path.leading_colon.is_some() {
            continue
        }
        if path.segments.len() != 1 {
            continue
        }
        let segment = path.segments.first().unwrap();
        if segment.ident != "repr" {
            continue
        }
        if segment.arguments != PathArguments::None {
            continue
        }
        let typ_paren = match syn::parse2::<Type>(tokens.clone()) {
            Ok(Type::Paren(TypeParen {
                elem,
                ..
            })) => *elem,
            _ => {
                continue
            }
        };
        let inner_path = match &typ_paren {
            Type::Path(t) => t,
            _ => {
                continue
            }
        };
        if let Some(seg) = inner_path.path.segments.last() {
            for t in &["u8","u16","u32","u64","usize","i8","i16","i32","i64","isize"] {
                if seg.ident == t {
                    index_type = typ_paren;
                    break
                }
            }
        }
    }

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
    let mut const_defs = Vec::new();
    let mut var_idx = 0usize;
    let mut has_additional_data = false;
    let mut prev_const_var_name = None;
    for variant in variants {
        use syn::Fields::*;

        if variant.get_variant_properties()?.disabled.is_some() {
            continue;
        }

        let ident = &variant.ident;
        let params = match &variant.fields {
            Unit => quote! {},
            Unnamed(fields) => {
                has_additional_data = true;
                let defaults = ::std::iter::repeat(quote!(::core::default::Default::default()))
                    .take(fields.unnamed.len());
                quote! { (#(#defaults),*) }
            }
            Named(fields) => {
                has_additional_data = true;
                let fields = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                quote! { {#(#fields: ::core::default::Default::default()),*} }
            }
        };

        let const_var_name = syn::parse_str::<Ident>(&format!("VARIANT{}", var_idx)).unwrap();
        let mut discriminant_found = false;
        if let Some((_eq,expr)) = &variant.discriminant {
            if let syn::Expr::Lit(expr_lit) = expr {
                if let syn::Lit::Int(int_lit) = &expr_lit.lit {
                    const_defs.push(quote! {pub const #const_var_name: #index_type = #int_lit;});
                    discriminant_found = true;
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        }
        if !discriminant_found {
            if let Some(prev) = &prev_const_var_name {
                const_defs.push(quote! {pub const #const_var_name: #index_type = #prev + 1;});
            } else {
                const_defs.push(quote! {pub const #const_var_name: #index_type = 0;});
            }
        }

        arms.push(quote! {v if v == #mod_name::#const_var_name => ::core::option::Option::Some(#name::#ident #params)});
        prev_const_var_name = Some(const_var_name);
        var_idx += 1;
    }

    arms.push(quote! { _ => ::core::option::Option::None });

    let const_impl = if has_additional_data {
        quote! {}
    } else {
        quote! {
            #vis const fn const_index(idx: #index_type) -> Option<#name #gen> {
                match idx {
                    #(#arms),*
                }
            }
        }
    };

    Ok(quote! {
        mod #mod_name {
            #(#const_defs)*
        }

        impl #name #gen {
            fn index(idx: #index_type) -> Option<#name #gen> {
                match idx {
                    #(#arms),*
                }
            }
            #const_impl
        }

    })
}
