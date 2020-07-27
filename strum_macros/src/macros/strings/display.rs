use proc_macro2::TokenStream;
use syn;

use crate::models::type_props::HasTypeProperties;
use crate::models::variant_props::HasStrumVariantProperties;

pub fn display_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("Display only works on Enums"),
    };

    let type_meta = ast.get_type_properties();
    let case_style = type_meta.case_style;

    let mut arms = Vec::new();
    for variant in variants {
        use syn::Fields::*;
        let ident = &variant.ident;
        let meta = variant.get_variant_properties();

        if meta.is_disabled {
            continue;
        }

        // Look at all the serialize attributes.
        let output = meta.get_preferred_name(type_meta.case_style);

        let params = match variant.fields {
            Unit => quote! {},
            Unnamed(..) => quote! { (..) },
            Named(..) => quote! { {..} },
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
