//! # Strum
//!
//! Strum is a set of macros and traits for working with
//! enums and strings easier in Rust.
//!
//! # Documentation
//!
//! The documentation for this crate is found in the `strum` crate.

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::Attribute;
use std::env;

#[proc_macro_derive(EnumString,attributes(strum))]
pub fn from_string(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = from_string_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.parse().unwrap()
}

#[proc_macro_derive(EnumIter,attributes(strum))]
pub fn enum_iter(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = enum_iter_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.parse().unwrap()
}

#[proc_macro_derive(EnumMessage,attributes(strum))]
pub fn enum_messages(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = enum_message_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.parse().unwrap()
}

#[proc_macro_derive(EnumProp,attributes(strum))]
pub fn enum_properties(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = enum_properties_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.parse().unwrap()
}

fn debug_print_generated(ast: &syn::DeriveInput, toks: &quote::Tokens) {
    let ident = ast.ident.as_ref();
    let debug = env::var("STRUM_DEBUG");
    if let Ok(s) = debug {
        if s == "1" {
            println!("{}", toks);
        }

        if s == ident {
            println!("{}", toks);
        }
    }
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
    let mut default =
        quote! { _ => ::std::result::Result::Err(::strum::ParseError::VariantNotFound) };
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
                    default => ::std::result::Result::Ok(#name::#ident (default.into()))
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

        arms.push(quote!{ #(#attrs)|* => ::std::result::Result::Ok(#name::#ident #params) });
    }

    arms.push(default);

    quote!{
        impl #impl_generics ::std::str::FromStr for #name #ty_generics #where_clause {
            type Err = ::strum::ParseError;
            fn from_str(s: &str) -> ::std::result::Result< #name #ty_generics , Self::Err> {
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
        let g = gen.ty_params.iter().map(|param| &param.ident).collect::<Vec<_>>();
        // quote!{ #ty_generics}
        quote!{ < ( #(#g),* ) > }
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

        arms.push(quote!{#idx => ::std::option::Option::Some(#name::#ident #params)});
    }

    arms.push(quote! { _ => ::std::option::Option::None });
    let iter_name = quote::Ident::from(&*format!("{}Iter", name));
    quote!{
        #vis struct #iter_name #ty_generics {
            idx: usize,
            marker: ::std::marker::PhantomData #phantom_data,
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
        _ => panic!("EnumMessage only works on Enums"),
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
            let tokens = quote!{ &#name::#ident #params => ::std::option::Option::Some(#msg) };
            arms.push(tokens.clone());

            if detailed_messages.is_none() {
                detailed_arms.push(tokens);
            }
        }

        if let Some(msg) = detailed_messages {
            let params = params.clone();
            // Push the simple message.
            detailed_arms.push(quote!{ &#name::#ident #params => ::std::option::Option::Some(#msg) });
        }
    }

    if arms.len() < variants.len() {
        arms.push(quote!{ _ => ::std::option::Option::None });
    }

    if detailed_arms.len() < variants.len() {
        detailed_arms.push(quote!{ _ => ::std::option::Option::None });
    }

    quote!{
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

fn extract_properties(ast: &syn::Variant) -> Vec<(&syn::Ident, &syn::Lit)> {
    use syn::*;
    ast.attrs
        .iter()
        .filter_map(|attr| {
            // Look for all the strum attributes
            if let &Attribute { value: MetaItem::List(ref ident, ref nested), .. } = attr {
                if ident == "strum" {
                    return Option::Some(nested);
                }
            }

            Option::None
        })
        .flat_map(|prop| prop)
        .filter_map(|prop| {
            // Look for all the recursive property attributes
            if let &NestedMetaItem::MetaItem(MetaItem::List(ref ident, ref nested)) = prop {
                if ident == "props" {
                    return Option::Some(nested);
                }
            }

            Option::None
        })
        .flat_map(|prop| prop)
        .filter_map(|prop| {
            // Only look at key value pairs
            if let &NestedMetaItem::MetaItem(MetaItem::NameValue(ref ident, ref value)) = prop {
                return Option::Some((ident, value));
            }

            Option::None
        })
        .collect()
}

fn enum_properties_inner(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("EnumProp only works on Enums"),
    };

    let mut arms = Vec::new();
    for variant in variants {
        let ident = &variant.ident;
        let mut string_arms = Vec::new();
        let mut bool_arms = Vec::new();
        let mut num_arms = Vec::new();
        // But you can disable the messages.
        if is_disabled(&variant.attrs) {
            continue;
        }

        use syn::VariantData::*;
        let params = match variant.data {
            Unit => quote::Ident::from(""),
            Tuple(..) => quote::Ident::from("(..)"),
            Struct(..) => quote::Ident::from("{..}"),
        };

        for (key, value) in extract_properties(&variant) {
            use syn::Lit::*;
            let key = key.as_ref();
            match *value {
                Str(ref s, ..) => {
                    string_arms.push(quote!{ #key => ::std::option::Option::Some( #s )})
                }
                Bool(b) => bool_arms.push(quote!{ #key => ::std::option::Option::Some( #b )}),
                Int(i, ..) => num_arms.push(quote!{ #key => ::std::option::Option::Some( #i )}),
                _ => {}
            }
        }

        string_arms.push(quote!{ _ => ::std::option::Option::None });
        bool_arms.push(quote!{ _ => ::std::option::Option::None });
        num_arms.push(quote!{ _ => ::std::option::Option::None });

        arms.push(quote!{
            &#name::#ident #params => {
                match prop {
                    #(#string_arms),*
                }
            }
        });
    }

    if arms.len() < variants.len() {
        arms.push(quote!{ _ => ::std::option::Option::None });
    }

    quote!{
        impl #impl_generics ::strum::EnumProperty for #name #ty_generics #where_clause {
            fn get_str(&self, prop: &str) -> ::std::option::Option<&'static str> {
                match self {
                    #(#arms),*
                }
            }
        }
    }
}
