use proc_macro2::TokenStream;
use syn;

use crate::helpers::case_style::CaseStyle;
use crate::helpers::{extract_meta, CaseStyleHelpers, MetaIteratorHelpers};

pub fn enum_variant_names_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("EnumVariantNames only works on Enums"),
    };

    // Derives for the generated enum
    let type_meta = extract_meta(&ast.attrs);
    let case_style = type_meta
        .find_unique_property("strum", "serialize_all")
        .map(|style| CaseStyle::from(style.as_ref()));

    let names = variants
        .iter()
        .map(|v| v.ident.convert_case(case_style))
        .collect::<Vec<_>>();

    quote! {
        impl #name {
            /// Return a slice containing the names of the variants of this enum
            #[allow(dead_code)]
            pub fn variants() -> &'static [&'static str] {
                &[
                    #(#names),*
                ]
            }
        }
    }
}
