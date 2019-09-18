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

mod helpers;
mod macros;

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

#[cfg_attr(
    not(feature = "verbose-enumstring-name"),
    proc_macro_derive(EnumString, attributes(strum))
)]
#[cfg_attr(
    feature = "verbose-enumstring-name",
    proc_macro_derive(StrumEnumString, attributes(strum))
)]
pub fn from_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = macros::from_string::from_string_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[cfg_attr(
    not(feature = "verbose-asrefstr-name"),
    proc_macro_derive(AsRefStr, attributes(strum))
)]
#[cfg_attr(
    feature = "verbose-asrefstr-name",
    proc_macro_derive(StrumAsRefStr, attributes(strum))
)]
pub fn as_ref_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = macros::as_ref_str::as_ref_str_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[cfg_attr(
    not(feature = "verbose-variant-names"),
    proc_macro_derive(EnumVariantNames, attributes(strum))
)]
#[cfg_attr(
    feature = "verbose-variant-names",
    proc_macro_derive(StrumEnumVariantNames, attributes(strum))
)]
pub fn variant_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = macros::enum_variant_names::enum_variant_names_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[cfg_attr(
    feature = "verbose-asstaticstr-name",
    proc_macro_derive(StrumAsStaticStr, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-asstaticstr-name"),
    proc_macro_derive(AsStaticStr, attributes(strum))
)]
pub fn as_static_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = macros::as_ref_str::as_static_str_inner(
        &ast,
        macros::as_ref_str::GenerateTraitVariant::AsStaticStr,
    );
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[cfg_attr(
    feature = "verbose-intostaticstr-name",
    proc_macro_derive(StrumIntoStaticStr, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-intostaticstr-name"),
    proc_macro_derive(IntoStaticStr, attributes(strum))
)]
pub fn into_static_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = macros::as_ref_str::as_static_str_inner(
        &ast,
        macros::as_ref_str::GenerateTraitVariant::From,
    );
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[cfg_attr(
    feature = "verbose-tostring-name",
    proc_macro_derive(StrumToString, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-tostring-name"),
    proc_macro_derive(ToString, attributes(strum))
)]
pub fn to_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = macros::to_string::to_string_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[cfg_attr(
    feature = "verbose-display-name",
    proc_macro_derive(StrumDisplay, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-display-name"),
    proc_macro_derive(Display, attributes(strum))
)]
pub fn display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = macros::display::display_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[cfg_attr(
    feature = "verbose-enumiter-name",
    proc_macro_derive(StrumEnumIter, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enumiter-name"),
    proc_macro_derive(EnumIter, attributes(strum))
)]
pub fn enum_iter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = macros::enum_iter::enum_iter_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[cfg_attr(
    feature = "verbose-enummessage-name",
    proc_macro_derive(StrumEnumMessage, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enummessage-name"),
    proc_macro_derive(EnumMessage, attributes(strum))
)]
pub fn enum_messages(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = macros::enum_messages::enum_message_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[cfg_attr(
    feature = "verbose-enumproperty-name",
    proc_macro_derive(StrumEnumProperty, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enumproperty-name"),
    proc_macro_derive(EnumProperty, attributes(strum))
)]
pub fn enum_properties(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = macros::enum_properties::enum_properties_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[cfg_attr(
    feature = "verbose-enumdiscriminants-name",
    proc_macro_derive(StrumEnumDiscriminants, attributes(strum, strum_discriminants))
)]
#[cfg_attr(
    not(feature = "verbose-enumdiscriminants-name"),
    proc_macro_derive(EnumDiscriminants, attributes(strum, strum_discriminants))
)]
pub fn enum_discriminants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let toks = macros::enum_discriminants::enum_discriminants_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}

#[cfg_attr(
    feature = "verbose-enumcount-name",
    proc_macro_derive(StrumEnumCount, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enumcount-name"),
    proc_macro_derive(EnumCount, attributes(strum))
)]
pub fn enum_count(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let toks = macros::enum_count::enum_count_inner(&ast);
    debug_print_generated(&ast, &toks);
    toks.into()
}
