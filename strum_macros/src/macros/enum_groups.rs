use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput};

use crate::helpers::{HasStrumVariantProperties, HasTypeProperties, non_enum_error};

pub fn enum_groups_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let type_properties = ast.get_type_properties()?;

    let derives = type_properties.discriminant_derives;

    let derives = quote! {
        #[derive(Debug, PartialEq, Eq, #(#derives),*)]
    };

    let enum_name = &ast.ident;
    let enum_name_group = format_ident!("{}Groups", enum_name);

    let mut all_types: Vec<_> = Vec::new();

    let _variants: Vec<_> = variants
        .iter()
        .filter_map(|variant| {
            let variant_properties = variant.get_variant_properties();

            if variant_properties.ok()?.disabled.is_some() {
                return None;
            }

            let mut is_named = false;

            let mut args: Vec<_> = Vec::new();
            let mut types: Vec<_> = Vec::new();
            for (i, field) in variant.fields.iter().enumerate() {
                let var;

                if let Some(_var) = field.clone().ident {
                    var = format_ident!("{}", _var);
                    is_named = true;
                } else {
                    var = format_ident!("_{}", i);
                }
                let fd = field.ty.to_token_stream();
                args.push(var);
                types.push(quote! {
                    #fd
                });
            }
            if args.len() > 0 {
                all_types.push((variant.ident.clone(), args, quote! {
                    (#(#types),*)
                }, is_named));
            }

            Some(0)
        })
                .collect();

    let mut arms: Vec<_> = Vec::new();
    let mut struct_groups: Vec<_> = Vec::new();
    let mut temp: HashSet<_> = HashSet::new();
    let groups: Vec<_> = all_types.iter().filter_map(|(ident, args, ty, is_named)| {
        let tys = ty.to_string().trim().to_lowercase();
        let id = format_ident!("g_{}", tys
            .replace(&[',', '(', ')', ':', '<', '>', '?', '[', ']', '{', '}'], "")
            .replace(&[' '], "_")
            .replace("__", "_"));
        let ref_args = quote! { (#(#args.clone()),*) };
        let argz = quote! { #(#args),* };
        let arm = if is_named.clone() {
            quote! {
                #enum_name::#ident {#argz} => { g.#id = Some(#ref_args); }
            }
        } else {
            quote! {
                #enum_name::#ident (#argz) => { g.#id = Some(#ref_args); }
            }
        };
        arms.push(arm);
        if !temp.insert(tys.clone()) { return None; }

        struct_groups.push(quote! {
            #id: Option<#ty>,
        });
        Some(quote! { #id })
    }).collect();

    Ok(quote! {

        #derives
        pub struct #enum_name_group {
            #(#struct_groups)*
        }

        impl #enum_name {
            fn get_groups(&self) -> #enum_name_group {
                let mut g = #enum_name_group {
                    #(#groups: None),*
                };
                match self {
                    #(#arms)*
                    _ => {}
                }
                return g;
            }
        }
    }.into())
}

