//! The strum_macros crate should be use in coordination with the `strum` crate.

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::Attribute;

#[proc_macro_derive(EnumString,attributes(strum))]
pub fn from_string(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = from_string_inner(&ast);
    toks.parse().unwrap()
}

#[proc_macro_derive(EnumIter,attributes(strum))]
pub fn enum_iter(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = enum_iter_inner(&ast);
    toks.parse().unwrap()
}

#[proc_macro_derive(EnumMessage,attributes(strum))]
pub fn enum_messages(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = enum_message_inner(&ast);
    toks.parse().unwrap()
}

fn extract_attrs<'a>(attrs: &'a [Attribute], attr: &str, prop: &str) -> Vec<&'a str> {
    attrs.iter()
        // Get all the attributes with our tag on them.
        .filter_map(|attribute| {
            use syn::MetaItem::*;
            if let List(ref i, ref nested) = attribute.value {
                if i == attr { Some(nested) } else { None }
            } else {
                None
            }
        })
        .flat_map(|nested| nested)
        // Get all the inner elements as long as they start with ser.
        .filter_map(|attribute| {
            use syn::NestedMetaItem::*;
            use syn::MetaItem::*;
            if let &MetaItem(NameValue(ref i, syn::Lit::Str(ref s, ..))) = attribute {
                if i == prop { Some(&**s) } else { None }
            } else {
                None
            }
        }).collect()
}

fn unique_attr<'a>(attrs: &'a [Attribute], attr: &str, prop: &str) -> Option<&'a str> {
    let mut curr = extract_attrs(attrs, attr, prop);
    if curr.len() > 1 {
        panic!("More than one property: {} found on variant", prop);
    }

    curr.pop()
}

fn is_disabled(attrs: &[Attribute]) -> bool {
    let v = extract_attrs(attrs, "strum", "disabled");
    match v.len() {
        0 => false,
        1 => v[0] == "true",
        _ => panic!("Can't have multiple values for 'disabled'"),
    }
}

fn from_string_inner(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("FromString only works on Enums"),
    };

    let mut has_default = false;
    let mut default = quote! { _ => Err(strum::ParseError::VariantNotFound) };
    let mut arms = Vec::new();
    for variant in variants {
        use syn::VariantData::*;
        let ident = &variant.ident;

        // Look at all the serialize attributes.
        let mut attrs = extract_attrs(&variant.attrs, "strum", "serialize");
        if is_disabled(&variant.attrs) {
            continue;
        }

        if let Some("true") = unique_attr(&variant.attrs, "strum", "default") {
            if has_default {
                panic!("Can't have multiple default variants");
            }

            if let Tuple(ref fields) = variant.data {
                if fields.len() != 1 {
                    panic!("Default only works on unit structs with a single String parameter");
                }

                default = quote!{
                    default => Ok(#name::#ident (default.into()))
                };
            } else {
                panic!("Default only works on unit structs with a single String parameter");
            }

            has_default = true;
            continue;
        }

        // If we don't have any custom variants, add the default name.
        if attrs.len() == 0 {
            attrs.push(ident.as_ref());
        }

        let params = match variant.data {
            Unit => quote::Ident::from(""),
            Tuple(ref fields) => {
                let default = fields.iter()
                    .map(|_| "Default::default()")
                    .collect::<Vec<_>>()
                    .join(", ");

                quote::Ident::from(&*format!("({})", default))
            }
            Struct(ref fields) => {
                let default = fields.iter()
                    .map(|field| {
                        format!("{}:{}", field.ident.as_ref().unwrap(), "Default::default()")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                quote::Ident::from(&*format!("{{{}}}", default))
            }
        };

        arms.push(quote!{ #(#attrs)|* => Ok(#name::#ident #params) });
    }

    arms.push(default);

    quote!{
        impl #impl_generics std::str::FromStr for #name #ty_generics #where_clause {
            type Err = strum::ParseError;
            fn from_str(s: &str) -> Result< #name #ty_generics , strum::ParseError> {
                match s {
                    #(#arms),*
                }
            }
        }
    }
}

fn enum_iter_inner(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let vis = &ast.vis;

    if gen.lifetimes.len() > 0 {
        panic!("Enum Iterator isn't supported on Enums with lifetimes. The resulting enums would \
                be unbounded.");
    }

    let phantom_data = if gen.ty_params.len() > 0 {
        quote!{ #ty_generics}
    } else {
        quote! { < () > }
    };

    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("EnumIter only works on Enums"),
    };

    let mut arms = Vec::new();
    let enabled = variants.iter().filter(|variant| !is_disabled(&variant.attrs));

    for (idx, variant) in enabled.enumerate() {
        use syn::VariantData::*;
        let ident = &variant.ident;
        let params = match variant.data {
            Unit => quote::Ident::from(""),
            Tuple(ref fields) => {
                let default = fields.iter()
                    .map(|_| "Default::default()")
                    .collect::<Vec<_>>()
                    .join(", ");

                quote::Ident::from(&*format!("({})", default))
            }
            Struct(ref fields) => {
                let default = fields.iter()
                    .map(|field| {
                        format!("{}:{}", field.ident.as_ref().unwrap(), "Default::default()")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                quote::Ident::from(&*format!("{{{}}}", default))
            }
        };

        arms.push(quote!{#idx => Some(#name::#ident #params)});
    }

    arms.push(quote! { _ => None });
    let iter_name = quote::Ident::from(&*format!("{}Iter", name));
    quote!{
        #vis struct #iter_name #ty_generics {
            idx: usize,
            marker: std::marker::PhantomData #phantom_data,
        }

        impl #impl_generics strum::IntoEnumIterator for #name #ty_generics #where_clause {
            type Iterator = #iter_name #ty_generics;
            fn iter() -> #iter_name #ty_generics {
                #iter_name {
                    idx:0,
                    marker: std::marker::PhantomData,
                }
            }
        }
        
        impl #impl_generics Iterator for #iter_name #ty_generics #where_clause {
            type Item = #name #ty_generics;
            
            fn next(&mut self) -> Option<#name #ty_generics> {
                use std::default::Default;
                let output = match self.idx {
                    #(#arms),*
                };

                self.idx += 1;
                output
            }
        }
    }
}

fn enum_message_inner(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("EnumHelp only works on Enums"),
    };

    let mut arms = Vec::new();
    let mut detailed_arms = Vec::new();
    let mut serializations = Vec::new();

    for variant in variants {
        let messages = unique_attr(&variant.attrs, "strum", "message");
        let detailed_messages = unique_attr(&variant.attrs, "strum", "detailed_message");
        let ident = &variant.ident;

        use syn::VariantData::*;
        let params = match variant.data {
            Unit => quote::Ident::from(""),
            Tuple(..) => quote::Ident::from("(..)"),
            Struct(..) => quote::Ident::from("{..}"),
        };

        // You can't disable getting the serializations.
        {
            let mut serialization_variants = extract_attrs(&variant.attrs, "strum", "serialize");
            if serialization_variants.len() == 0 {
                serialization_variants.push(ident.as_ref());
            }

            let count = serialization_variants.len();
            serializations.push(quote!{
                &#name::#ident #params => {
                    static ARR: [&'static str; #count] = [#(#serialization_variants),*];
                    &ARR
                }
            });
        }

        // But you can disable the messages.
        if is_disabled(&variant.attrs) {
            continue;
        }

        if let Some(msg) = messages {
            let params = params.clone();

            // Push the simple message.
            let tokens = quote!{ &#name::#ident #params => Some(#msg) };
            arms.push(tokens.clone());

            if detailed_messages.is_none() {
                detailed_arms.push(tokens);
            }
        }

        if let Some(msg) = detailed_messages {
            let params = params.clone();
            // Push the simple message.
            detailed_arms.push(quote!{ &#name::#ident #params => Some(#msg) });
        }
    }

    if arms.len() < variants.len() {
        arms.push(quote!{ _ => None });
    }

    if detailed_arms.len() < variants.len() {
        detailed_arms.push(quote!{ _ => None });
    }

    quote!{
        impl #impl_generics strum::EnumMessage for #name #ty_generics #where_clause {
            fn get_message(&self) -> Option<&str> {
                match self {
                    #(#arms),*
                }
            }

            fn get_detailed_message(&self) -> Option<&str> {
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
