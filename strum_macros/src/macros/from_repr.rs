use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::metadata_impl::{FromReprTokens, MetadataImpl};

pub fn from_repr_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let vis = &ast.vis;

    let mut metadata = MetadataImpl::new(ast)?.use_from_repr();

    let discriminant_type = metadata.discriminant_type();
    metadata.generate()?;
    let FromReprTokens {
        constant_defs,
        match_arms,
    } = &metadata.from_repr().as_ref().unwrap();
    let (impl_generics, ty_generics, where_clause) = metadata.generics_split();

    let const_if_possible = if metadata.has_additional_data {
        quote! {}
    } else {
        #[rustversion::before(1.46)]
        fn filter_by_rust_version(s: TokenStream) -> TokenStream {
            quote! {}
        }

        #[rustversion::since(1.46)]
        fn filter_by_rust_version(s: TokenStream) -> TokenStream {
            s
        }
        filter_by_rust_version(quote! { const })
    };

    Ok(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #[doc = "Try to create [Self] from the raw representation"]
            #vis #const_if_possible fn from_repr(discriminant: #discriminant_type) -> Option<#name #ty_generics> {
                #(#constant_defs)*
                match discriminant {
                    #(#match_arms),*
                }
            }
        }
    })
}
