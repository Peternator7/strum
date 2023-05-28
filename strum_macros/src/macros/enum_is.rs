use crate::helpers::{non_enum_error, HasStrumVariantProperties};
use heck::ToSnakeCase;
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
        .filter_map(|variant| {
            if variant.get_variant_properties().ok()?.disabled.is_some() {
                return None;
            }

            let variant_name = &variant.ident;
            let fn_name = format_ident!("is_{}", snakify(&variant_name.to_string()));

            Some(quote! {
                #[must_use]
                #[inline]
                pub const fn #fn_name(&self) -> bool {
                    match self {
                        &#enum_name::#variant_name { .. } => true,
                        _ => false
                    }
                }
            })
        })
        .collect();

    Ok(quote! {
        impl #enum_name {
            #(#variants)*
        }
    }
    .into())
}

/// heck doesn't treat numbers as new words, but this function does.
/// E.g. for input `Hello2You`, heck would output `hello2_you`, and snakify would output `hello_2_you`.
fn snakify(s: &str) -> String {
    let mut output: Vec<char> = s.to_string().to_snake_case().chars().collect();
    let mut num_starts = vec![];
    for (pos, c) in output.iter().enumerate() {
        if c.is_digit(10) && pos != 0 && !output[pos - 1].is_digit(10) {
            num_starts.push(pos);
        }
    }
    // need to do in reverse, because after inserting, all chars after the point of insertion are off
    for i in num_starts.into_iter().rev() {
        output.insert(i, '_')
    }
    output.into_iter().collect()
}
