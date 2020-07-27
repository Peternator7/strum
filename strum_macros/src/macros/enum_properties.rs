use proc_macro2::TokenStream;
use syn;

use crate::models::HasStrumVariantProperties;

// fn extract_properties(meta: &[Meta]) -> Vec<(&syn::Path, &syn::Lit)> {
//     meta.iter()
//         // Filter down to the strum(..) attribute
//         .filter_map(|meta| meta.expect_metalist())
//         .filter(|list| list.path.is_ident("strum"))
//         .flat_map(|list| list.expand_inner())
//         // Filter down to the `strum(props(..))` attribute
//         .filter_map(|meta| meta.expect_metalist())
//         .filter(|inner_list| inner_list.path.is_ident("props"))
//         .flat_map(|inner_list| inner_list.expand_inner())
//         // Expand all the pairs `strum(props(key = value, ..))`
//         .filter_map(|prop| match *prop {
//             syn::Meta::NameValue(syn::MetaNameValue {
//                 ref path, ref lit, ..
//             }) => Some((path, lit)),
//             _ => None,
//         })
//         .collect()
// }

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
        let meta = variant.get_variant_properties();
        let mut string_arms = Vec::new();
        let mut bool_arms = Vec::new();
        let mut num_arms = Vec::new();
        // But you can disable the messages.
        if meta.is_disabled {
            continue;
        }

        use syn::Fields::*;
        let params = match variant.fields {
            Unit => quote! {},
            Unnamed(..) => quote! { (..) },
            Named(..) => quote! { {..} },
        };

        for (key, value) in meta.string_props {
            string_arms.push(quote! { #key => ::std::option::Option::Some( #value )})
        }

        string_arms.push(quote! { _ => ::std::option::Option::None });
        bool_arms.push(quote! { _ => ::std::option::Option::None });
        num_arms.push(quote! { _ => ::std::option::Option::None });

        arms.push(quote! {
            &#name::#ident #params => {
                match prop {
                    #(#string_arms),*
                }
            }
        });
    }

    if arms.len() < variants.len() {
        arms.push(quote! { _ => ::std::option::Option::None });
    }

    quote! {
        impl #impl_generics ::strum::EnumProperty for #name #ty_generics #where_clause {
            fn get_str(&self, prop: &str) -> ::std::option::Option<&'static str> {
                match self {
                    #(#arms),*
                }
            }
        }
    }
}
