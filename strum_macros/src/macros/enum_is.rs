use crate::helpers::{case_style::snakify, non_enum_error, HasStrumVariantProperties};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

pub fn enum_is_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let enum_name = &ast.ident;
    let variants: Vec<_> = variants
        .iter()
        .filter_map(|variant| {
            let variant_props = variant.get_variant_properties().ok()?;

            if variant_props.disabled.is_some() {
                return None;
            }

            let ident = &variant.ident;
            let name = variant_props
                .name
                .as_ref()
                .map(|s| s.value())
                .unwrap_or_else(|| snakify(&ident.to_string()));

            let fn_name = format_ident!("is_{}", name);
            let doc_comment =
                format!("Returns [true] if the enum is [{enum_name}::{ident}] otherwise [false]",);

            Some(quote! {
                #[must_use]
                #[inline]
                #[doc = #doc_comment]
                pub const fn #fn_name(&self) -> bool {
                    match self {
                        &#enum_name::#ident { .. } => true,
                        _ => false
                    }
                }
            })
        })
        .collect();

    Ok(quote! {
        impl #impl_generics #enum_name  #ty_generics #where_clause {
            #(#variants)*
        }
    })
}
