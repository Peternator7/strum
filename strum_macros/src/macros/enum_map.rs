use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{spanned::Spanned, Data, DeriveInput, Fields};

use crate::helpers::{non_enum_error, snakify, HasStrumVariantProperties};

pub fn enum_map_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let vis = &ast.vis;
    let mut doc_comment = format!("A map over the variants of [{}]", name);

    if gen.lifetimes().count() > 0 {
        return Err(syn::Error::new(
            Span::call_site(),
            "This macro doesn't support enums with lifetimes.",
        ));
    }

    let Data::Enum(data_enum) = &ast.data else {
        return Err(non_enum_error())
    };
    let map_name = format_ident!("{}Map", name);

    let variants = &data_enum.variants;

    // the identifiers of each variant, in PascalCase
    let mut pascal_idents = Vec::new();
    // the identifiers of each struct field, in snake_case
    let mut snake_idents = Vec::new();
    // match arms in the form `MyEnumMap::Variant => &self.variant,`
    let mut get_matches = Vec::new();
    // match arms in the form `MyEnumMap::Variant => &mut self.variant,`
    let mut get_matches_mut = Vec::new();
    // match arms in the form `MyEnumMap::Variant => self.variant = new_value`
    let mut set_matches = Vec::new();
    // struct fields of the form `variant: func(MyEnum::Variant),*
    let mut closure_fields = Vec::new();
    // struct fields of the form `variant: func(MyEnum::Variant, self.variant),`
    let mut transform_fields = Vec::new();

    // identifiers for disabled variants
    let mut disabled_variants = Vec::new();
    // match arms for disabled variants
    let mut disabled_matches = Vec::new();

    for variant in variants {
        // skip disabled variants
        if variant.get_variant_properties()?.disabled.is_some() {
            let disabled_ident = &variant.ident;
            let panic_message =
                format!("Can't use `{disabled_ident}` with `{map_name}` - variant is disabled for Strum features");
            disabled_variants.push(disabled_ident);
            disabled_matches.push(quote!(#name::#disabled_ident => panic!(#panic_message),));
            continue;
        }

        // Error on fields with data
        let Fields::Unit = &variant.fields else {
            return Err(syn::Error::new(
                variant.fields.span(),
                "This macro doesn't support enums with non-unit variants",
            ))
        };

        let pascal_case = &variant.ident;
        let snake_case = snakify(&pascal_case.to_string());

        get_matches.push(quote! {#name::#pascal_case => &self.#snake_case,});
        get_matches_mut.push(quote! {#name::#pascal_case => &mut self.#snake_case,});
        set_matches.push(quote! {#name::#pascal_case => self.#snake_case = new_value,});
        closure_fields.push(quote! {#snake_case: func(#name::#pascal_case),});
        transform_fields.push(quote! {#snake_case: func(#name::#pascal_case, &self.#snake_case),});
        pascal_idents.push(pascal_case);
        snake_idents.push(snake_case);
    }

    // if the index operation can panic, add that to the documentation
    if !disabled_variants.is_empty() {
        doc_comment.push_str(&format!(
            "\n# Panics\nIndexing `{map_name}` with any of the following variants will cause a panic:"
        ));
        for variant in disabled_variants {
            doc_comment.push_str(&format!("\n\n- `{name}::{variant}`"));
        }
    }

    let doc_new = format!("Create a new {map_name} with a value for each variant of {name}");
    let doc_closure =
        format!("Create a new {map_name} by running a function on each variant of `{name}`");
    let doc_transform = format!("Create a new `{map_name}` by running a function on each variant of `{name}` and the corresponding value in the current `{map_name}`");
    let doc_filled = format!("Create a new `{map_name}` with the same value in each field.");
    let doc_option_all = format!("Converts `{map_name}<Option<T>>` into `Option<{map_name}<T>>`. Returns `Some` if all fields are `Some`, otherwise returns `None`.");
    let doc_result_all_ok = format!("Converts `{map_name}<Result<T, E>>` into `Option<{map_name}>`. Returns `Some` if all fields are `Ok`, otherwise returns `None`.");

    Ok(quote! {
        #[doc = #doc_comment]
        #[allow(
            missing_copy_implementations,
        )]
        #[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
        #vis struct #map_name<T> {
            #(#snake_idents: T,)*
        }

        impl<T: Clone> #map_name<T> {
            #[doc = #doc_filled]
            #vis fn filled(value: T) -> #map_name<T> {
                #map_name {
                    #(#snake_idents: value.clone(),)*
                }
            }
        }

        impl<T> #map_name<T> {
            #[doc = #doc_new]
            #vis fn new(
                #(#snake_idents: T,)*
            ) -> #map_name<T> {
                #map_name {
                    #(#snake_idents,)*
                }
            }

            #[doc = #doc_closure]
            #vis fn from_closure<F: Fn(#name)->T>(func: F) -> #map_name<T> {
              #map_name {
                #(#closure_fields)*
              }
            }

            #[doc = #doc_transform]
            #vis fn transform<U, F: Fn(#name, &T)->U>(&self, func: F) -> #map_name<U> {
              #map_name {
                #(#transform_fields)*
              }
            }

        }

        impl<T> core::ops::Index<#name> for #map_name<T> {
            type Output = T;

            fn index(&self, idx: #name) -> &T {
                match idx {
                    #(#get_matches)*
                    #(#disabled_matches)*
                }
            }
        }

        impl<T> core::ops::IndexMut<#name> for #map_name<T> {
            fn index_mut(&mut self, idx: #name) -> &mut T {
                match idx {
                    #(#get_matches_mut)*
                    #(#disabled_matches)*
                }
            }
        }

        impl<T> #map_name<Option<T>> {
            #[doc = #doc_option_all]
            #vis fn all(self) -> Option<#map_name<T>> {
                if let #map_name {
                    #(#snake_idents: Some(#snake_idents),)*
                } = self {
                    Some(#map_name {
                        #(#snake_idents,)*
                    })
                } else {
                    None
                }
            }
        }

        impl<T, E> #map_name<Result<T, E>> {
            #[doc = #doc_result_all_ok]
            #vis fn all_ok(self) -> Option<#map_name<T>> {
                if let #map_name {
                    #(#snake_idents: Ok(#snake_idents),)*
                } = self {
                    Some(#map_name {
                        #(#snake_idents,)*
                    })
                } else {
                    None
                }
            }
        }
    })
}
