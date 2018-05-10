
use quote;
use syn;

use helpers::{unique_attr, extract_attrs, extract_meta, is_disabled};

pub fn to_string_inner(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("ToString only works on Enums"),
    };

    let mut arms = Vec::new();
    for variant in variants {
        use syn::Fields::*;
        let ident = &variant.ident;
        let meta = extract_meta(&variant.attrs);

        if is_disabled(&meta) {
            continue;
        }

        // Look at all the serialize attributes.
        let output = if let Some(n) = unique_attr(&meta, "strum", "to_string") {
            n
        } else {
            let mut attrs = extract_attrs(&meta, "strum", "serialize");
            // We always take the longest one. This is arbitary, but is *mostly* deterministic
            attrs.sort_by_key(|s| s.len());
            if let Some(n) = attrs.pop() {
                n
            } else {
                ident.to_string()
            }
        };

        let params = match variant.fields {
            Unit => quote!{},
            Unnamed(..) => quote!{ (..) },
            Named(..) => quote!{ {..} },
        };

        arms.push(quote!{ #name::#ident #params => ::std::string::String::from(#output) });
    }

    if arms.len() < variants.len() {
        arms.push(quote!{ _ => panic!("to_string() called on disabled variant.")})
    }

    quote!{
        impl #impl_generics ::std::string::ToString for #name #ty_generics #where_clause {
            fn to_string(&self) -> ::std::string::String {
                match *self {
                    #(#arms),*
                }
            }
        }
    }
}
