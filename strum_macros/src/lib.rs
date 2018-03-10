//! # Strum
//!
//! Strum is a set of macros and traits for working with
//! enums and strings easier in Rust.
//!
//! # Documentation
//!
//! The documentation for this crate is found in the `strum` crate.

#![recursion_limit="128"]

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

mod helpers;
mod as_ref_str;
mod display;
mod to_string;
mod from_string;
mod enum_iter;
mod enum_messages;
mod enum_properties;

use proc_macro::TokenStream;
use std::env;

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

#[proc_macro_derive(EnumString,attributes(strum))]
pub fn from_string(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = from_string::from_string_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.parse().unwrap()
}

#[proc_macro_derive(AsRefStr,attributes(strum))]
pub fn as_ref_str(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = as_ref_str::as_ref_str_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.parse().unwrap()
}

#[proc_macro_derive(ToString,attributes(strum))]
pub fn to_string(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = to_string::to_string_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.parse().unwrap()
}

#[proc_macro_derive(Display,attributes(strum))]
pub fn display(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = display::display_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.parse().unwrap()
}

#[proc_macro_derive(EnumIter,attributes(strum))]
pub fn enum_iter(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = enum_iter::enum_iter_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.parse().unwrap()
}

#[proc_macro_derive(EnumMessage,attributes(strum))]
pub fn enum_messages(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = enum_messages::enum_message_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.parse().unwrap()
}

#[proc_macro_derive(EnumProperty,attributes(strum))]
pub fn enum_properties(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let toks = enum_properties::enum_properties_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.parse().unwrap()
}
