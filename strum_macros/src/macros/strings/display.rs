use proc_macro2::TokenStream;
use syn;

use crate::helpers::case_style::CaseStyle;
use helpers::{extract_meta, CaseStyleHelpers, MetaIteratorHelpers};

pub fn display_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("Display only works on Enums"),
    };

    let type_meta = extract_meta(&ast.attrs);
    let case_style = type_meta
        .find_unique_property("strum", "serialize_all")
        .map(|style| CaseStyle::from(style.as_ref()));

    let mut arms = Vec::new();
    for variant in variants {
        use syn::Fields::*;
        let ident = &variant.ident;
        let meta = extract_meta(&variant.attrs);

        if meta.is_disabled() {
            continue;
        }

        // Look at all the serialize attributes.
        let output = if let Some(n) = meta.find_unique_property("strum", "to_string") {
            n
        } else {
            let mut attrs = meta.find_properties("strum", "serialize");
            // We always take the longest one. This is arbitary, but is *mostly* deterministic
            attrs.sort_by_key(|s| s.len());
            if let Some(n) = attrs.pop() {
                n
            } else {
                ident.convert_case(case_style)
            }
        };

        let params = match variant.fields {
            Unit => quote! {},
            Unnamed(..) => quote! { (..) },
            Named(..) => quote! { {..} },
        };

        arms.push(quote! { #name::#ident #params => f.write_str(#output) });
    }

    if arms.len() < variants.len() {
        arms.push(quote! { _ => panic!("fmt() called on disabled variant.")})
    }

    quote! {
        impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                match *self {
                    #(#arms),*
                }
            }
        }
    }
}
