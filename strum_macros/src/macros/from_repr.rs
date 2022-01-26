use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::DeriveInput;

use crate::helpers::metadata_impl::{FromReprTokens, MetadataImpl};

pub fn from_repr_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();
    let vis = &ast.vis;

    if gen.lifetimes().count() > 0 {
        return Err(syn::Error::new(
            Span::call_site(),
            "This macro doesn't support enums with lifetimes. \
             The resulting enums would be unbounded.",
        ));
    }

    let mut metadata = MetadataImpl::new(ast).use_from_repr();
    let discriminant_type = metadata.discriminant_type();
    metadata.generate()?;
    let FromReprTokens {
        constant_defs,
        match_arms,
    } = &metadata.from_repr().as_ref().unwrap();

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
