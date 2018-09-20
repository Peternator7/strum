use proc_macro2::{Span, TokenStream};
use syn;

use helpers::{extract_meta, extract_meta_idents, unique_meta_ident, unique_meta_list};

pub fn enum_discriminants_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let vis = &ast.vis;

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("EnumDiscriminants only works on Enums"),
    };

    let type_meta = extract_meta(&ast.attrs);
    let discriminant_meta = unique_meta_list(&type_meta, "strum_discriminants");
    let derives =
        discriminant_meta.map_or_else(|| vec![], |meta| extract_meta_idents(&[meta], "derive"));

    let derives = quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, #(#derives),*)]
    };

    let default_name = syn::Ident::new(
        &format!("{}Discriminants", name.to_string()),
        Span::call_site(),
    );
    let discriminants_name = discriminant_meta
        .map(|meta| unique_meta_ident(&[meta], "name").unwrap_or(&default_name))
        .unwrap_or(&default_name);

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
        #vis enum #discriminants_name {
            #(#discriminants),*
        }
    }
}
