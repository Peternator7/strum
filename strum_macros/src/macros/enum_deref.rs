use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsUnnamed, Meta};

use crate::helpers::{get_new_type_variant, no_associated_deref_type_specified, non_enum_error};

pub fn enum_deref_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let attrs = &ast.attrs;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let target_type = attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("strum_deref_target") {
                if let Meta::List(list) = &attr.meta {
                    return Some(&list.tokens);
                }
            }
            None
        })
        .ok_or_else(no_associated_deref_type_specified)?;

    let idents: Vec<Ident> = variants
        .iter()
        .map(|variant| get_new_type_variant(variant).map(|(variant_ident, _)| variant_ident))
        .collect::<syn::Result<_>>()?;

    Ok(quote! {
        impl #impl_generics std::ops::Deref for #name #ty_generics #where_clause {
            type Target = #target_type;

            fn deref(&self) -> &Self::Target {
                match self {
                    #( #name::#idents (ref inner) => inner, )*
                }
            }
        }

        impl #impl_generics std::ops::DerefMut for #name #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                match self {
                    #( #name::#idents (ref mut inner) => inner, )*
                }
            }
        }
    })
}
