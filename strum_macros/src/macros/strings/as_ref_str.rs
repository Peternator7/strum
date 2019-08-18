use proc_macro2::TokenStream;
use syn;

use crate::helpers::case_style::CaseStyle;
use helpers::{extract_meta, CaseStyleHelpers, MetaIteratorHelpers};

fn get_arms(ast: &syn::DeriveInput) -> Vec<TokenStream> {
    let name = &ast.ident;
    let mut arms = Vec::new();
    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("This macro only works on Enums"),
    };

    let type_meta = extract_meta(&ast.attrs);
    let case_style = type_meta
        .find_unique_property("strum", "serialize_all")
        .map(|style| CaseStyle::from(style.as_ref()));

    for variant in variants {
        use syn::Fields::*;
        let ident = &variant.ident;
        let meta = extract_meta(&variant.attrs);

        if meta.is_disabled() {
            continue;
        }

        // Look at all the serialize attributes.
        // Use `to_string` attribute (not `as_ref_str` or something) to keep things consistent
        // (i.e. always `enum.as_ref().to_string() == enum.to_string()`).
        let output = if let Some(n) = meta.find_unique_property("strum", "to_string") {
            n
        } else {
            let mut attrs = meta.find_properties("strum", "serialize");
            // We always take the longest one. This is arbitary, but is *mostly* deterministic
            attrs.sort_by_key(|s| s.len());
            if let Some(n) = attrs.pop() {
                n
            } else {
                ident.convert_case(case_style)
            }
        };

        let params = match variant.fields {
            Unit => quote! {},
            Unnamed(..) => quote! { (..) },
            Named(..) => quote! { {..} },
        };

        arms.push(quote! { #name::#ident #params => #output });
    }

    if arms.len() < variants.len() {
        arms.push(quote! {
        _ => panic!("AsRef::<str>::as_ref() or AsStaticRef::<str>::as_static() \
                     called on disabled variant.")
        })
    }

    arms
}

pub fn as_ref_str_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let arms = get_arms(ast);
    quote! {
        impl #impl_generics ::std::convert::AsRef<str> for #name #ty_generics #where_clause {
            fn as_ref(&self) -> &str {
                match *self {
                    #(#arms),*
                }
            }
        }
    }
}

pub enum GenerateTraitVariant {
    AsStaticStr,
    From,
}

pub fn as_static_str_inner(
    ast: &syn::DeriveInput,
    trait_variant: GenerateTraitVariant,
) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let arms = get_arms(ast);

    let mut generics = ast.generics.clone();
    generics
        .params
        .push(syn::GenericParam::Lifetime(syn::LifetimeDef::new(
            parse_quote!('_derivative_strum),
        )));
    let (impl_generics2, _, _) = generics.split_for_impl();
    let arms2 = arms.clone();
    let arms3 = arms.clone();
    match trait_variant {
        GenerateTraitVariant::AsStaticStr => {
            quote! {
                impl #impl_generics ::strum::AsStaticRef<str> for #name #ty_generics #where_clause {
                    fn as_static(&self) -> &'static str {
                        match *self {
                            #(#arms),*
                        }
                    }
                }
            }
        }
        GenerateTraitVariant::From => {
            quote! {
            impl #impl_generics ::std::convert::From<#name #ty_generics> for &'static str #where_clause {
                fn from(x: #name #ty_generics) -> &'static str {
                    match x {
                        #(#arms2),*
                    }
                }
            }
            impl #impl_generics2 ::std::convert::From<&'_derivative_strum #name #ty_generics> for &'static str #where_clause {
                fn from(x: &'_derivative_strum #name #ty_generics) -> &'static str {
                    match *x {
                        #(#arms3),*
                    }
                }
            }
            }
        }
    }
}
