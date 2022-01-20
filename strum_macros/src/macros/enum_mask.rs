use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, PathArguments, Type, TypeParen};

use crate::helpers::{non_enum_error, HasStrumVariantProperties, HasTypeProperties};

pub fn enum_mask_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();
    let attrs = &ast.attrs;

    let mut discriminant_type: Type = syn::parse("usize".parse().unwrap()).unwrap();
    for attr in attrs {
        let path = &attr.path;
        let tokens = &attr.tokens;
        if path.leading_colon.is_some() {
            continue;
        }
        if path.segments.len() != 1 {
            continue;
        }
        let segment = path.segments.first().unwrap();
        if segment.ident != "repr" {
            continue;
        }
        if segment.arguments != PathArguments::None {
            continue;
        }
        let typ_paren = match syn::parse2::<Type>(tokens.clone()) {
            Ok(Type::Paren(TypeParen { elem, .. })) => *elem,
            _ => continue,
        };
        let inner_path = match &typ_paren {
            Type::Path(t) => t,
            _ => continue,
        };
        if let Some(seg) = inner_path.path.segments.last() {
            for t in &[
                "u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize",
            ] {
                if seg.ident == t {
                    discriminant_type = typ_paren;
                    break;
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
    let mut constant_defs = Vec::new();
    let mut prev_const_var_ident = None;
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

        use heck::ToShoutySnakeCase;
        let const_var_str = format!("{}_DISCRIMINANT", variant.ident).to_shouty_snake_case();
        let const_var_ident = format_ident!("{}", const_var_str);

        let const_val_expr = match &variant.discriminant {
            Some((_, expr)) => quote! { #expr },
            None => match &prev_const_var_ident {
                Some(prev) => quote! { #prev + 1 },
                None => quote! { 0 },
            },
        };

        constant_defs.push(quote! {const #const_var_ident: #discriminant_type = #const_val_expr;});
        arms.push(quote! {v if v == #const_var_ident => ::core::option::Option::Some(#name::#ident #params)});

        prev_const_var_ident = Some(const_var_ident);
    }

    arms.push(quote! { _ => ::core::option::Option::None });

    // FIXME this should check that discriminants are comprised of unique bits.

    let type_properties = ast.get_type_properties()?;
    let strum_module_path = type_properties.crate_module_path();
    Ok(quote! {
        impl #impl_generics EnumRepr for #name #ty_generics #where_clause {
            #[doc = "The Repr type"]
            type Repr = #discriminant_type;
            #[doc = "The Opaque Repr type"]
            type OpaqueRepr = #strum_module_path::OpaqueRepr<Self>;
            #[doc = "The Enum type (Self for the enum, the enum for OpaqueRepr)"]
            type EnumT = Self;

            fn to_repr(self) -> #discriminant_type {
               self as #discriminant_type
            }

            fn opaque(self) -> #strum_module_path::OpaqueRepr<Self> {
                #strum_module_path::OpaqueRepr::new(self)
            }

            fn cvt_from_repr(discriminant: #discriminant_type) -> Option<Self> {
                #(#constant_defs)*
                match discriminant {
                    #(#arms),*
                }
            }
        }
    })
}
