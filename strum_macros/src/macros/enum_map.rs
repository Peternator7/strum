use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Fields, Ident};

use crate::helpers::{non_enum_error, HasStrumVariantProperties};

pub fn enum_map_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let vis = &ast.vis;
    let doc_comment = format!("A map over the variants of [{}]", name);

    if gen.lifetimes().count() > 0 {
        return Err(syn::Error::new(
            Span::call_site(),
            "This macro doesn't support enums with lifetimes.",
        ));
    }

    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let mut arms = Vec::new();
    let mut idx = 0usize;
    for variant in variants {
        if variant.get_variant_properties()?.disabled.is_some() {
            continue;
        }

        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => {}
            _ => {
                return Err(syn::Error::new(
                    variant.fields.span(),
                    "This macro doesn't support enums with non-unit variants",
                ))
            }
        };

        arms.push(quote! {#name::#ident => #idx});
        idx += 1;
    }

    let variant_count = arms.len();
    let map_name = syn::parse_str::<Ident>(&format!("{}Map", name)).unwrap();

    // Create a string literal "MyEnumMap" to use in the debug impl.
    let map_name_debug_struct =
        syn::parse_str::<syn::LitStr>(&format!("\"{}\"", map_name)).unwrap();

    Ok(quote! {
        #[doc = #doc_comment]
        #[allow(
            missing_copy_implementations,
        )]
        #vis struct #map_name<T> {
            content: [T; #variant_count]
        }

        impl<T: Default> #map_name<T> {
            fn new() -> Self {
                Self {
                    content: Default::default()
                }
            }
        }

        impl<T> core::fmt::Debug for #map_name<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                // We don't know if the variants implement debug themselves so the only thing we
                // can really show is how many elements are left.
                f.debug_struct(#map_name_debug_struct)
                    .field("len", &#variant_count)
                    .finish()
            }
        }

        impl<T> core::ops::Index<#name> for #map_name<T> {
            type Output = T;

            fn index(&self, idx: #name) -> &T {
                &self.content[{match idx {
                    #(#arms),*
                }}]
            }
        }

        impl<T> core::ops::IndexMut<#name> for #map_name<T> {
            fn index_mut(&mut self, idx: #name) -> &mut T {
                self.content.index_mut({match idx {
                    #(#arms),*
                }})
            }
        }

        impl<T: Clone> core::iter::IntoIterator for #map_name<T> {
            type Item = (#name, T);
            type IntoIter = <Vec<(#name, T)> as core::iter::IntoIterator>::IntoIter;

            fn into_iter(self) -> Self::IntoIter {
                use strum::IntoEnumIterator;
                let pairs: Vec<(#name, T)> = #name::iter().map(|variant| (variant, self[variant].clone())).collect();
                pairs.into_iter()
            }
        }

        impl<T: Clone> Clone for #map_name<T> {
            fn clone(&self) -> Self {
                Self {
                    content: self.content.clone()
                }
            }
        }
    })
}
