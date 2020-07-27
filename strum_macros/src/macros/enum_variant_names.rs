use proc_macro2::TokenStream;
use syn;

use crate::helpers::CaseStyleHelpers;
use crate::models::{HasStrumVariantProperties, HasTypeProperties};

pub fn enum_variant_names_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("EnumVariantNames only works on Enums"),
    };

    // Derives for the generated enum
    let type_meta = ast.get_type_properties();

    let names = variants
        .iter()
        .map(|v| v.ident.convert_case(type_meta.case_style))
        .collect::<Vec<_>>();

    quote! {
        impl #impl_generics ::strum::VariantNames for #name #ty_generics #where_clause {
            const VARIANTS: &'static [&'static str] = &[ #(#names),* ];
        }
    }
}
