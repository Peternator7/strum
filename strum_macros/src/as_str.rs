
use quote;
use syn;

use helpers::{unique_attr, extract_attrs, is_disabled};

pub fn as_str_inner(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("AsStr only works on Enums"),
    };

    let mut arms = Vec::new();
    for variant in variants {
        use syn::VariantData::*;
        let ident = &variant.ident;

        if is_disabled(&variant.attrs) {
            continue;
        }

        // Look at all the serialize attributes.
        let output = if let Some(n) = unique_attr(&variant.attrs, "strum", "as_str") {
            n
        } else if let Some(n) = unique_attr(&variant.attrs, "strum", "to_string") {
            n
        } else {
            let mut attrs = extract_attrs(&variant.attrs, "strum", "serialize");
            // We always take the longest one. This is arbitary, but is *mostly* deterministic
            attrs.sort_by_key(|s| -(s.len() as isize));
            if let Some(n) = attrs.first() {
                n
            } else {
                ident.as_ref()
            }
        };

        let params = match variant.data {
            Unit => quote::Ident::from(""),
            Tuple(..) => quote::Ident::from("(..)"),
            Struct(..) => quote::Ident::from("{..}"),
        };

        arms.push(quote!{ #name::#ident #params => #output });
    }

    if arms.len() < variants.len() {
        arms.push(quote!{ _ => panic!("to_string() called on disabled variant.")})
    }

    quote!{
        impl #impl_generics ::std::convert::AsRef<str> for #name #ty_generics #where_clause {
            fn as_ref(&self) -> &str {
                match *self {
                    #(#arms),*
                }
            }
        }
    }
}
