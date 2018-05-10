
use quote;
use syn;

use helpers::{unique_attr, extract_attrs, is_disabled};

pub fn display_inner(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("Display only works on Enums"),
    };

    let mut arms = Vec::new();
    for variant in variants {
        use syn::Fields::*;
        let ident = &variant.ident;

        if is_disabled(&variant.attrs) {
            continue;
        }

        // Look at all the serialize attributes.
        let output = if let Some(n) = unique_attr(&variant.attrs, "strum", "to_string") {
            n
        } else {
            let mut attrs = extract_attrs(&variant.attrs, "strum", "serialize");
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

        arms.push(quote!{ #name::#ident #params => f.write_str(#output) });
    }

    if arms.len() < variants.len() {
        arms.push(quote!{ _ => panic!("fmt() called on disabled variant.")})
    }

    quote!{
        impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                match *self {
                    #(#arms),*
                }
            }
        }
    }
}
