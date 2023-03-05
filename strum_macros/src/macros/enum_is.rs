use crate::helpers::non_enum_error;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

pub fn enum_is_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let enum_name = &ast.ident;

    let variants: Vec<_> = variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let fn_name = format_ident!("is_{}", variant_name.to_string().to_case(Case::Snake));

            match &variant.fields {
                syn::Fields::Named(_) => {
                    quote! {
                        #[must_use]
                        #[inline]
                        pub const fn #fn_name(&self) -> bool {
                            matches!(self, &#enum_name::#variant_name { .. })
                        }
                    }
                }
                syn::Fields::Unnamed(values) => {
                    let underscores = values.unnamed.iter().map(|_| quote! {_});
                    quote! {
                        #[must_use]
                        #[inline]
                        pub const fn #fn_name(&self) -> bool {
                            matches!(self, &#enum_name::#variant_name (#(#underscores),*))
                        }
                    }
                }
                syn::Fields::Unit => {
                    quote! {
                        #[must_use]
                        #[inline]
                        pub const fn #fn_name(&self) -> bool {
                            matches!(self, &#enum_name::#variant_name)
                        }
                    }
                }
            }
        })
        .collect();

    Ok(quote! {
        impl #enum_name {
            #(#variants)*
        }
    }
    .into())
}
