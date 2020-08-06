use proc_macro2::TokenStream;
use syn;

use crate::helpers::{HasStrumVariantProperties, HasTypeProperties};

pub fn display_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("Display only works on Enums"),
    };

    let type_properties = ast.get_type_properties();

    let mut arms = Vec::new();
    for variant in variants {
        let ident = &variant.ident;
        let variant_properties = variant.get_variant_properties();

        if variant_properties.is_disabled {
            continue;
        }

        // Look at all the serialize attributes.
        let output = variant_properties.get_preferred_name(type_properties.case_style);

        let params = match variant.fields {
            syn::Fields::Unit => quote! {},
            syn::Fields::Unnamed(..) => quote! { (..) },
            syn::Fields::Named(..) => quote! { {..} },
        };

        arms.push(quote! { #name::#ident #params => f.pad(#output) });
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
