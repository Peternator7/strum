use proc_macro2::TokenStream;
use syn;

use crate::helpers::{HasStrumVariantProperties, HasTypeProperties};

pub fn enum_message_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("EnumMessage only works on Enums"),
    };

    let type_properties = ast.get_type_properties();

    let mut arms = Vec::new();
    let mut detailed_arms = Vec::new();
    let mut serializations = Vec::new();

    for variant in variants {
        let variant_properties = variant.get_variant_properties();
        let messages = variant_properties.message.as_ref();
        let detailed_messages = variant_properties.detailed_message.as_ref();
        let ident = &variant.ident;

        use syn::Fields::*;
        let params = match variant.fields {
            Unit => quote! {},
            Unnamed(..) => quote! { (..) },
            Named(..) => quote! { {..} },
        };

        // You can't disable getting the serializations.
        {
            let serialization_variants =
                variant_properties.get_serializations(type_properties.case_style);

            let count = serialization_variants.len();
            serializations.push(quote! {
                &#name::#ident #params => {
                    static ARR: [&'static str; #count] = [#(#serialization_variants),*];
                    &ARR
                }
            });
        }

        // But you can disable the messages.
        if variant_properties.is_disabled {
            continue;
        }

        if let Some(msg) = messages {
            let params = params.clone();

            // Push the simple message.
            let tokens = quote! { &#name::#ident #params => ::std::option::Option::Some(#msg) };
            arms.push(tokens.clone());

            if detailed_messages.is_none() {
                detailed_arms.push(tokens);
            }
        }

        if let Some(msg) = detailed_messages {
            let params = params.clone();
            // Push the simple message.
            detailed_arms
                .push(quote! { &#name::#ident #params => ::std::option::Option::Some(#msg) });
        }
    }

    if arms.len() < variants.len() {
        arms.push(quote! { _ => ::std::option::Option::None });
    }

    if detailed_arms.len() < variants.len() {
        detailed_arms.push(quote! { _ => ::std::option::Option::None });
    }

    quote! {
        impl #impl_generics ::strum::EnumMessage for #name #ty_generics #where_clause {
            fn get_message(&self) -> ::std::option::Option<&str> {
                match self {
                    #(#arms),*
                }
            }

            fn get_detailed_message(&self) -> ::std::option::Option<&str> {
                match self {
                    #(#detailed_arms),*
                }
            }

            fn get_serializations(&self) -> &[&str] {
                match self {
                    #(#serializations),*
                }
            }
        }
    }
}
