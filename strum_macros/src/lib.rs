//! # Strum
//!
//! Strum is a set of macros and traits for working with
//! enums and strings easier in Rust.
//!
//! # Documentation
//!
//! The documentation for this crate is found in the `strum` crate.

#![recursion_limit = "128"]

extern crate heck;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;

mod as_ref_str;
mod case_style;
mod display;
mod enum_discriminants;
mod enum_iter;
mod enum_messages;
mod enum_properties;
mod from_string;
mod helpers;
mod to_string;

use proc_macro2::TokenStream;
use std::env;

fn debug_print_generated(ast: &syn::DeriveInput, toks: &TokenStream) {
    let debug = env::var("STRUM_DEBUG");
    if let Ok(s) = debug {
        if s == "1" {
            println!("{}", toks);
        }

        if ast.ident == s {
            println!("{}", toks);
        }
    }
}

#[proc_macro_derive(EnumString, attributes(strum))]
pub fn from_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = from_string::from_string_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[proc_macro_derive(AsRefStr, attributes(strum))]
pub fn as_ref_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = as_ref_str::as_ref_str_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[proc_macro_derive(AsStaticStr, attributes(strum))]
pub fn as_static_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = as_ref_str::as_static_str_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[proc_macro_derive(ToString, attributes(strum))]
pub fn to_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = to_string::to_string_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[proc_macro_derive(Display, attributes(strum))]
pub fn display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = display::display_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[proc_macro_derive(EnumIter, attributes(strum))]
pub fn enum_iter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = enum_iter::enum_iter_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[proc_macro_derive(EnumMessage, attributes(strum))]
pub fn enum_messages(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = enum_messages::enum_message_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[proc_macro_derive(EnumProperty, attributes(strum))]
pub fn enum_properties(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = enum_properties::enum_properties_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[proc_macro_derive(EnumDiscriminants, attributes(strum, strum_discriminants))]
pub fn enum_discriminants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = enum_discriminants::enum_discriminants_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}
