extern crate strum;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::Attribute;

#[proc_macro_derive(FromString,attributes(strum))]
pub fn from_string(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = from_string_inner(&ast);
    toks.parse().unwrap()
}

#[proc_macro_derive(EnumIter,attributes(strum))]
pub fn unit_enum_iter(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = enum_iter(&ast);
    println!("{:?}", toks);
    toks.parse().unwrap()
}

#[proc_macro_derive(EnumHelp,attributes(strum))]
pub fn unit_enum_help_messages(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = enum_help(&ast);
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
    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("FromString only works on Enums"),
    };

    let mut arms = Vec::new();
    for variant in variants {
        use syn::VariantData::*;
        let ident = &variant.ident;

        // Look at all the serialize attributes.
        let mut attrs = extract_attrs(&variant.attrs, "strum", "serialize");
        if is_disabled(&variant.attrs) {
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
    arms.push(quote! { _ => Err(strum::ParseError::VariantNotFound) });
    quote!{
        impl std::str::FromStr for #name {
            type Err = strum::ParseError;
            fn from_str(s: &str) -> Result<#name,strum::ParseError> {
                match s {
                    #(#arms),*
                }
            }
        }
    }
}

fn enum_iter(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
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
        struct #iter_name {
            idx: usize,
        }

        impl strum::IntoEnumIterator for #name {
            type Iterator = #iter_name;
            fn iter() -> #iter_name {
                #iter_name {
                    idx:0,
                }
            }
        }
        
        impl Iterator for #iter_name {
            type Item = #name;
            
            fn next(&mut self) -> Option<#name> {
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

fn enum_help(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("EnumHelp only works on Enums"),
    };

    let mut arms = Vec::new();
    let mut detailed_arms = Vec::new();
    let enabled = variants.iter().filter(|variant| !is_disabled(&variant.attrs));
    for variant in enabled {
        let mut help = extract_attrs(&variant.attrs, "strum", "help");
        let ident = &variant.ident;
        if help.len() > 1 {
            panic!("More than one help message on {}::{}", name, ident);
        }

        if let Some(msg) = help.pop() {
            use syn::VariantData::*;
            let params = match variant.data {
                Unit => quote::Ident::from(""),
                Tuple(..) => quote::Ident::from("(..)"),
                Struct(..) => quote::Ident::from("{{..}}"),
            };

            // Push the simple message.
            arms.push(quote!{ &#name::#ident #params => Some(#msg) });

            // Create the more complex message.
            let mut serialize = extract_attrs(&variant.attrs, "strum", "serialize");
            if serialize.len() == 0 {
                serialize.push(ident.as_ref());
            }

            let detailed_msg = format!("{}: {}", serialize.join(", "), msg);
            detailed_arms.push(quote!{&#name::#ident #params => Some(#detailed_msg) });
        }
    }

    arms.push(quote!{ _ => None });
    detailed_arms.push(quote!{ _ => None });
    quote!{
        impl #name {
            pub fn get_help(&self) -> Option<&str> {
                match self {
                    #(#arms),*
                }
            }

            pub fn get_detailed_help(&self) -> Option<&str> {
                match self {
                    #(#detailed_arms),*
                }
            }
        }
    }
}
