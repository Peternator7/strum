use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitByteStr};

use crate::helpers::{
    missing_parse_err_attr_error, non_enum_error, non_unit_variant_error,
    HasStrumVariantProperties, HasTypeProperties,
};

pub fn from_string_const_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let vis = &ast.vis;
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let type_properties = ast.get_type_properties()?;
    let strum_module_path = type_properties.crate_module_path();

    let mut match_arms = Vec::new();
    let mut has_case_insensitive_arm = false;
    for variant in variants {
        let ident = &variant.ident;
        let variant_properties = variant.get_variant_properties()?;

        if variant_properties.disabled.is_some() {
            continue;
        }

        if let Some(kw) = variant_properties.default {
            return Err(syn::Error::new_spanned(
                kw,
                "Default is not supported by EnumStringConst because the default \
                 variant can't be constructed in a const fn",
            ));
        }

        if !matches!(variant.fields, Fields::Unit) {
            return Err(non_unit_variant_error());
        }

        let is_ascii_case_insensitive = variant_properties
            .ascii_case_insensitive
            .unwrap_or(type_properties.ascii_case_insensitive);

        for serialization in variant_properties.get_serializations(type_properties.case_style) {
            // `&str` can't be matched in a const fn, so match on `s.as_bytes()` with
            // byte string literals instead.
            let serialization =
                LitByteStr::new(serialization.value().as_bytes(), serialization.span());

            if is_ascii_case_insensitive {
                has_case_insensitive_arm = true;
                match_arms.push(
                    quote! { s if eq_ignore_ascii_case(s, #serialization) => #name::#ident, },
                );
            } else {
                match_arms.push(quote! { #serialization => #name::#ident, });
            }
        }
    }

    // Same rules as EnumString for custom error types, except that without a default
    // variant, parsing is never infallible so the attributes are all-or-nothing.
    let (err_ty, not_found_expr) =
        match (&type_properties.parse_err_ty, &type_properties.parse_err_fn) {
            (Some(ty), Some(f)) => (quote! { #ty }, quote! { #f(s) }),
            (None, None) => (
                quote! { #strum_module_path::ParseError },
                quote! { #strum_module_path::ParseError::VariantNotFound },
            ),
            _ => return Err(missing_parse_err_attr_error()),
        };

    // `str::eq_ignore_ascii_case` isn't a const fn, so roll our own over the byte
    // slices. It's defined inside the generated fn to keep it out of the caller's scope.
    let eq_ignore_ascii_case_fn = if has_case_insensitive_arm {
        quote! {
            const fn eq_ignore_ascii_case(lhs: &[u8], rhs: &[u8]) -> bool {
                if lhs.len() != rhs.len() {
                    return false;
                }

                let mut i = 0;
                while i < lhs.len() {
                    if !lhs[i].eq_ignore_ascii_case(&rhs[i]) {
                        return false;
                    }

                    i += 1;
                }

                true
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #[allow(clippy::use_self)]
        #[automatically_derived]
        impl #impl_generics #name #ty_generics #where_clause {
            #[doc = "Try to create [Self] from a string in a const context"]
            #[inline]
            #vis const fn from_str_const(s: &str) -> ::core::result::Result<#name #ty_generics, #err_ty> {
                #eq_ignore_ascii_case_fn

                ::core::result::Result::Ok(match s.as_bytes() {
                    #(#match_arms)*
                    _ => return ::core::result::Result::Err(#not_found_expr),
                })
            }
        }
    })
}
