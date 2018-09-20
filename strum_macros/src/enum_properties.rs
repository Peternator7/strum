use proc_macro2::TokenStream;
use syn;
use syn::Meta;

use helpers::{extract_meta, is_disabled};

fn extract_properties(meta: &[Meta]) -> Vec<(&syn::Ident, &syn::Lit)> {
    use syn::{MetaList, MetaNameValue, NestedMeta};
    meta.iter()
        .filter_map(|meta| match *meta {
            Meta::List(MetaList {
                ref ident,
                ref nested,
                ..
            }) => {
                if ident == "strum" {
                    Some(nested)
                } else {
                    None
                }
            }
            _ => None,
        }).flat_map(|prop| prop)
        .filter_map(|prop| match *prop {
            NestedMeta::Meta(Meta::List(MetaList {
                ref ident,
                ref nested,
                ..
            })) => {
                if ident == "props" {
                    Some(nested)
                } else {
                    None
                }
            }
            _ => None,
        }).flat_map(|prop| prop)
        // Only look at key value pairs
        .filter_map(|prop| match *prop {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                ref ident, ref lit, ..
            })) => Some((ident, lit)),
            _ => None,
        }).collect()
}

pub fn enum_properties_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("EnumProp only works on Enums"),
    };

    let mut arms = Vec::new();
    for variant in variants {
        let ident = &variant.ident;
        let meta = extract_meta(&variant.attrs);
        let mut string_arms = Vec::new();
        let mut bool_arms = Vec::new();
        let mut num_arms = Vec::new();
        // But you can disable the messages.
        if is_disabled(&meta) {
            continue;
        }

        use syn::Fields::*;
        let params = match variant.fields {
            Unit => quote!{},
            Unnamed(..) => quote!{ (..) },
            Named(..) => quote!{ {..} },
        };

        for (key, value) in extract_properties(&meta) {
            use syn::Lit::*;
            let key = key.to_string();
            match value {
                Str(ref s, ..) => {
                    string_arms.push(quote!{ #key => ::std::option::Option::Some( #s )})
                }
                Bool(b) => bool_arms.push(quote!{ #key => ::std::option::Option::Some( #b )}),
                Int(i, ..) => num_arms.push(quote!{ #key => ::std::option::Option::Some( #i )}),
                _ => {}
            }
        }

        string_arms.push(quote!{ _ => ::std::option::Option::None });
        bool_arms.push(quote!{ _ => ::std::option::Option::None });
        num_arms.push(quote!{ _ => ::std::option::Option::None });

        arms.push(quote!{
            &#name::#ident #params => {
                match prop {
                    #(#string_arms),*
                }
            }
        });
    }

    if arms.len() < variants.len() {
        arms.push(quote!{ _ => ::std::option::Option::None });
    }

    quote!{
        impl #impl_generics ::strum::EnumProperty for #name #ty_generics #where_clause {
            fn get_str(&self, prop: &str) -> ::std::option::Option<&'static str> {
                match self {
                    #(#arms),*
                }
            }
        }
    }
}
