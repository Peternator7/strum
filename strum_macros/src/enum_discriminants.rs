use proc_macro2::{Span, TokenStream};
use syn;

use helpers::{extract_meta, extract_meta_attrs, unique_meta_attr};

pub fn enum_discriminants_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let vis = &ast.vis;

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("EnumDiscriminants only works on Enums"),
    };

    let type_meta = extract_meta(&ast.attrs);
    let discriminant_derives = extract_meta_attrs(&type_meta, "strum_discriminants_derive");
    let derives = quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, #(#discriminant_derives),*)]
    };

    let default_name = syn::Ident::new(
        &format!("{}Discriminants", name.to_string()),
        Span::call_site(),
    );
    let enum_discriminants_name =
        unique_meta_attr(&type_meta, "strum_discriminants_name").unwrap_or(&default_name);

    let mut discriminants = Vec::new();
    for variant in variants {
        let ident = &variant.ident;

        // Don't copy across the "strum" meta attribute.
        let attrs = variant.attrs.iter().filter(|attr| {
            attr.interpret_meta().map_or(true, |meta| match meta {
                syn::Meta::List(ref metalist) => metalist.ident != "strum",
                _ => true,
            })
        });

        discriminants.push(quote!{ #(#attrs)* #ident });
    }

    quote!{
        /// Auto-generated discriminant enum variants
        #derives
        #vis enum #enum_discriminants_name {
            #(#discriminants),*
        }
    }
}
