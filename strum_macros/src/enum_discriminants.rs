use proc_macro2::{Span, TokenStream};
use syn;

use helpers::{extract_list_metas, extract_meta, get_meta_ident, get_meta_list, unique_meta_list};

pub fn enum_discriminants_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let vis = &ast.vis;

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("EnumDiscriminants only works on Enums"),
    };

    // Derives for the generated enum
    let type_meta = extract_meta(&ast.attrs);
    let discriminant_attrs = unique_meta_list(type_meta.iter(), "strum_discriminants")
        .map(|meta| extract_list_metas(meta).collect::<Vec<_>>());
    let derives = discriminant_attrs.as_ref().map_or_else(
        || vec![],
        |meta| {
            get_meta_list(meta.iter().map(|&m| m), "derive")
                .flat_map(extract_list_metas)
                .filter_map(get_meta_ident)
                .collect::<Vec<_>>()
        },
    );

    let derives = quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, #(#derives),*)]
    };

    // Work out the name
    let default_name = syn::Ident::new(
        &format!("{}Discriminants", name.to_string()),
        Span::call_site(),
    );
    let discriminants_name = discriminant_attrs
        .as_ref()
        .and_then(|meta| unique_meta_list(meta.iter().map(|&m| m), "name"))
        .map(extract_list_metas)
        .and_then(|metas| metas.filter_map(get_meta_ident).next())
        .unwrap_or(&default_name);

    // Add the variants without fields, but exclude the `strum` meta item
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
