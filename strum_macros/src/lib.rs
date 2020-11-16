//! # Strum
//!
//! Strum is a set of macros and traits for working with
//! enums and strings easier in Rust.
//!

#![recursion_limit = "128"]

extern crate proc_macro;

mod helpers;
mod macros;

use proc_macro2::TokenStream;
use std::env;
use syn::DeriveInput;

fn debug_print_generated(ast: &DeriveInput, toks: &TokenStream) {
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

/// Converts strings to enum variants based on their name.
///
/// auto-derives `std::str::FromStr` on the enum. Each variant of the enum will match on it's own name.
/// This can be overridden using `serialize="DifferentName"` or `to_string="DifferentName"`
/// on the attribute as shown below.
/// Multiple deserializations can be added to the same variant. If the variant contains additional data,
/// they will be set to their default values upon deserialization.
///
/// The `default` attribute can be applied to a tuple variant with a single data parameter. When a match isn't
/// found, the given variant will be returned and the input string will be captured in the parameter.
///
/// Note that the implementation of `FromStr` by default only matches on the name of the
/// variant. There is an option to match on different case conversions through the
/// `#[strum(serialize_all = "snake_case")]` type attribute.
///
/// See the [Additional Attributes](../strum/additional_attributes/index.html)
/// Section for more information on using this feature.
///
/// # Example howto use EnumString
/// ```
/// use std::str::FromStr;
/// use strum_macros::EnumString;
///
/// #[derive(Debug, PartialEq, EnumString)]
/// enum Color {
///     Red,
///     // The Default value will be inserted into range if we match "Green".
///     Green {
///         range: usize,
///     },
///
///     // We can match on multiple different patterns.
///     #[strum(serialize = "blue", serialize = "b")]
///     Blue(usize),
///
///     // Notice that we can disable certain variants from being found
///     #[strum(disabled)]
///     Yellow,
/// }
///
/// /*
/// //The generated code will look like:
/// impl std::str::FromStr for Color {
///     type Err = ::strum::ParseError;
///
///     fn from_str(s: &str) -> ::std::result::Result<Color, Self::Err> {
///         match s {
///             "Red" => ::std::result::Result::Ok(Color::Red),
///             "Green" => ::std::result::Result::Ok(Color::Green { range:Default::default() }),
///             "blue" | "b" => ::std::result::Result::Ok(Color::Blue(Default::default())),
///             _ => ::std::result::Result::Err(::strum::ParseError::VariantNotFound),
///         }
///     }
/// }
/// */
///
/// // simple from string
/// let color_variant = Color::from_str("Red").unwrap();
/// assert_eq!(Color::Red, color_variant);
/// // short version works too
/// let color_variant = Color::from_str("b").unwrap();
/// assert_eq!(Color::Blue(0), color_variant);
/// // was disabled for parsing = returns parse-error
/// let color_variant = Color::from_str("Yellow");
/// assert!(color_variant.is_err());
/// // however the variant is still normally usable
/// println!("{:?}", Color::Yellow);
/// ```
#[cfg_attr(
    not(feature = "verbose-enumstring-name"),
    proc_macro_derive(EnumString, attributes(strum))
)]
#[cfg_attr(
    feature = "verbose-enumstring-name",
    proc_macro_derive(StrumEnumString, attributes(strum))
)]
pub fn from_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks =
        macros::from_string::from_string_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// Converts enum variants to `&'static str`.
///
/// Implements `AsRef<str>` on your enum using the same rules as
/// `Display` for determining what string is returned. The difference is that `as_ref()` returns
/// a `&str` instead of a `String` so you don't allocate any additional memory with each call.
///
/// ```
/// // You need to bring the AsRef trait into scope to use it
/// use std::convert::AsRef;
/// use strum_macros::AsRefStr;
///
/// #[derive(AsRefStr, Debug)]
/// enum Color {
///     #[strum(serialize = "redred")]
///     Red,
///     Green {
///         range: usize,
///     },
///     Blue(usize),
///     Yellow,
/// }
///
/// // uses the serialize string for Display
/// let red = Color::Red;
/// assert_eq!("redred", red.as_ref());
/// // by default the variants Name
/// let yellow = Color::Yellow;
/// assert_eq!("Yellow", yellow.as_ref());
/// // or for string formatting
/// println!(
///     "blue: {} green: {}",
///     Color::Blue(10).as_ref(),
///     Color::Green { range: 42 }.as_ref()
/// );
/// ```
#[cfg_attr(
    not(feature = "verbose-asrefstr-name"),
    proc_macro_derive(AsRefStr, attributes(strum))
)]
#[cfg_attr(
    feature = "verbose-asrefstr-name",
    proc_macro_derive(StrumAsRefStr, attributes(strum))
)]
pub fn as_ref_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks =
        macros::as_ref_str::as_ref_str_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// Implements Strum::VariantNames which adds an associated constant `VARIANTS` which is an array of discriminant names.
///
/// Adds an `impl` block for the `enum` that adds a static `VARIANTS` array of `&'static str` that are the discriminant names.
/// This will respect the `serialize_all` attribute on the `enum` (like `#[strum(serialize_all = "snake_case")]`.
///
/// ```
/// // import the macros needed
/// use strum_macros::{EnumString, EnumVariantNames};
/// // You need to import the trait, to have access to VARIANTS
/// use strum::VariantNames;
///
/// #[derive(Debug, EnumString, EnumVariantNames)]
/// #[strum(serialize_all = "kebab_case")]
/// enum Color {
///     Red,
///     Blue,
///     Yellow,
///     RebeccaPurple,
/// }
/// assert_eq!(["red", "blue", "yellow", "rebecca-purple"], Color::VARIANTS);
/// ```
#[cfg_attr(
    not(feature = "verbose-variant-names"),
    proc_macro_derive(EnumVariantNames, attributes(strum))
)]
#[cfg_attr(
    feature = "verbose-variant-names",
    proc_macro_derive(StrumEnumVariantNames, attributes(strum))
)]
pub fn variant_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks = macros::enum_variant_names::enum_variant_names_inner(&ast)
        .unwrap_or_else(|err| err.to_compile_error());
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
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks = macros::as_ref_str::as_static_str_inner(
        &ast,
        macros::as_ref_str::GenerateTraitVariant::AsStaticStr,
    )
    .unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// Implements `From<MyEnum> for &'static str` on an enum.
///
/// Implements `From<YourEnum>` and `From<&'a YourEnum>` for `&'static str`. This is
/// useful for turning an enum variant into a static string.
/// The Rust `std` provides a blanket impl of the reverse direction - i.e. `impl Into<&'static str> for YourEnum`.
///
/// ```
/// use strum_macros::IntoStaticStr;
///
/// #[derive(IntoStaticStr)]
/// enum State<'a> {
///     Initial(&'a str),
///     Finished,
/// }
///
/// fn verify_state<'a>(s: &'a str) {
///     let mut state = State::Initial(s);
///     // The following won't work because the lifetime is incorrect:
///     // let wrong: &'static str = state.as_ref();
///     // using the trait implemented by the derive works however:
///     let right: &'static str = state.into();
///     assert_eq!("Initial", right);
///     state = State::Finished;
///     let done: &'static str = state.into();
///     assert_eq!("Finished", done);
/// }
///
/// verify_state(&"hello world".to_string());
/// ```
#[cfg_attr(
    feature = "verbose-intostaticstr-name",
    proc_macro_derive(StrumIntoStaticStr, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-intostaticstr-name"),
    proc_macro_derive(IntoStaticStr, attributes(strum))
)]
pub fn into_static_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks = macros::as_ref_str::as_static_str_inner(
        &ast,
        macros::as_ref_str::GenerateTraitVariant::From,
    )
    .unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// implements `std::string::ToString` on en enum
///
/// ```
/// // You need to bring the ToString trait into scope to use it
/// use std::string::ToString;
/// use strum_macros;
///
/// #[derive(strum_macros::ToString, Debug)]
/// enum Color {
///     #[strum(serialize = "redred")]
///     Red,
///     Green {
///         range: usize,
///     },
///     Blue(usize),
///     Yellow,
/// }
///
/// // uses the serialize string for Display
/// let red = Color::Red;
/// assert_eq!(String::from("redred"), red.to_string());
/// // by default the variants Name
/// let yellow = Color::Yellow;
/// assert_eq!(String::from("Yellow"), yellow.to_string());
/// ```
#[cfg_attr(
    feature = "verbose-tostring-name",
    proc_macro_derive(StrumToString, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-tostring-name"),
    proc_macro_derive(ToString, attributes(strum))
)]
pub fn to_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks =
        macros::to_string::to_string_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// Converts enum variants to strings.
///
/// Deriving `Display` on an enum prints out the given enum. This enables you to perform round
/// trip style conversions from enum into string and back again for unit style variants. `Display`
/// choose which serialization to used based on the following criteria:
///
/// 1. If there is a `to_string` property, this value will be used. There can only be one per variant.
/// 1. Of the various `serialize` properties, the value with the longest length is chosen. If that
///    behavior isn't desired, you should use `to_string`.
/// 1. The name of the variant will be used if there are no `serialize` or `to_string` attributes.
///
/// ```
/// // You need to bring the ToString trait into scope to use it
/// use std::string::ToString;
/// use strum_macros::Display;
///
/// #[derive(Display, Debug)]
/// enum Color {
///     #[strum(serialize = "redred")]
///     Red,
///     Green {
///         range: usize,
///     },
///     Blue(usize),
///     Yellow,
/// }
///
/// // uses the serialize string for Display
/// let red = Color::Red;
/// assert_eq!(String::from("redred"), format!("{}", red));
/// // by default the variants Name
/// let yellow = Color::Yellow;
/// assert_eq!(String::from("Yellow"), yellow.to_string());
/// // or for string formatting
/// println!(
///     "blue: {} green: {}",
///     Color::Blue(10),
///     Color::Green { range: 42 }
/// );
/// ```
#[cfg_attr(
    feature = "verbose-display-name",
    proc_macro_derive(StrumDisplay, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-display-name"),
    proc_macro_derive(Display, attributes(strum))
)]
pub fn display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks = macros::display::display_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// Creates a new type that iterates of the variants of an enum.
///
/// Iterate over the variants of an Enum. Any additional data on your variants will be set to `Default::default()`.
/// The macro implements `strum::IntoEnumIter` on your enum and creates a new type called `YourEnumIter` that is the iterator object.
/// You cannot derive `EnumIter` on any type with a lifetime bound (`<'a>`) because the iterator would surely
/// create [unbounded lifetimes](https://doc.rust-lang.org/nightly/nomicon/unbounded-lifetimes.html).
///
/// ```
///
/// // You need to bring the trait into scope to use it!
/// use strum::IntoEnumIterator;
/// use strum_macros::EnumIter;
///
/// #[derive(EnumIter, Debug, PartialEq)]
/// enum Color {
///     Red,
///     Green { range: usize },
///     Blue(usize),
///     Yellow,
/// }
///
/// // It's simple to iterate over the variants of an enum.
/// for color in Color::iter() {
///     println!("My favorite color is {:?}", color);
/// }
///
/// let mut ci = Color::iter();
/// assert_eq!(Some(Color::Red), ci.next());
/// assert_eq!(Some(Color::Green {range: 0}), ci.next());
/// assert_eq!(Some(Color::Blue(0)), ci.next());
/// assert_eq!(Some(Color::Yellow), ci.next());
/// assert_eq!(None, ci.next());
/// ```
#[cfg_attr(
    feature = "verbose-enumiter-name",
    proc_macro_derive(StrumEnumIter, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enumiter-name"),
    proc_macro_derive(EnumIter, attributes(strum))
)]
pub fn enum_iter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks =
        macros::enum_iter::enum_iter_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// Add a verbose message to an enum variant.
///
/// Encode strings into the enum itself. The `strum_macros::EmumMessage` macro implements the `strum::EnumMessage` trait.
/// `EnumMessage` looks for `#[strum(message="...")]` attributes on your variants.
/// You can also provided a `detailed_message="..."` attribute to create a seperate more detailed message than the first.
/// ```
/// // You need to bring the trait into scope to use it
/// use strum::EnumMessage;
/// use strum_macros;
///
/// #[derive(strum_macros::EnumMessage, Debug)]
/// #[allow(dead_code)]
/// enum Color {
///     #[strum(message = "Red", detailed_message = "This is very red")]
///     Red,
///     #[strum(message = "Simply Green")]
///     Green { range: usize },
///     #[strum(serialize = "b", serialize = "blue")]
///     Blue(usize),
/// }
///
/// // Generated code looks like more or less like this:
/// /*
/// impl ::strum::EnumMessage for Color {
///     fn get_message(&self) -> ::std::option::Option<&str> {
///         match self {
///             &Color::Red => ::std::option::Option::Some("Red"),
///             &Color::Green {..} => ::std::option::Option::Some("Simply Green"),
///             _ => None
///         }
///     }
///
///     fn get_detailed_message(&self) -> ::std::option::Option<&str> {
///         match self {
///             &Color::Red => ::std::option::Option::Some("This is very red"),
///             &Color::Green {..}=> ::std::option::Option::Some("Simply Green"),
///             _ => None
///         }
///     }
///
///     fn get_serializations(&self) -> &[&str] {
///         match self {
///             &Color::Red => {
///                 static ARR: [&'static str; 1] = ["Red"];
///                 &ARR
///             },
///             &Color::Green {..}=> {
///                 static ARR: [&'static str; 1] = ["Green"];
///                 &ARR
///             },
///             &Color::Blue (..) => {
///                 static ARR: [&'static str; 2] = ["b", "blue"];
///                 &ARR
///             },
///         }
///     }
/// }
/// */
///
/// let c = Color::Red;
/// assert_eq!("Red", c.get_message().unwrap());
/// assert_eq!("This is very red", c.get_detailed_message().unwrap());
/// assert_eq!(["Red"], c.get_serializations());
/// ```
#[cfg_attr(
    feature = "verbose-enummessage-name",
    proc_macro_derive(StrumEnumMessage, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enummessage-name"),
    proc_macro_derive(EnumMessage, attributes(strum))
)]
pub fn enum_messages(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks = macros::enum_messages::enum_message_inner(&ast)
        .unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// Add custom properties to enum variants.
///
/// Enables the encoding of arbitary constants into enum variants. This method
/// currently only supports adding additional string values. Other types of literals are still
/// experimental in the rustc compiler. The generated code works by nesting match statements.
/// The first match statement matches on the type of the enum, and the inner match statement
/// matches on the name of the property requested. This design works well for enums with a small
/// number of variants and properties, but scales linearly with the number of variants so may not
/// be the best choice in all situations.
///
/// ```
///
/// use strum_macros;
/// // bring the trait into scope
/// use strum::EnumProperty;
///
/// #[derive(strum_macros::EnumProperty, Debug)]
/// #[allow(dead_code)]
/// enum Color {
///     #[strum(props(Red = "255", Blue = "255", Green = "255"))]
///     White,
///     #[strum(props(Red = "0", Blue = "0", Green = "0"))]
///     Black,
///     #[strum(props(Red = "0", Blue = "255", Green = "0"))]
///     Blue,
///     #[strum(props(Red = "255", Blue = "0", Green = "0"))]
///     Red,
///     #[strum(props(Red = "0", Blue = "0", Green = "255"))]
///     Green,
/// }
///
/// let my_color = Color::Red;
/// let display = format!(
///     "My color is {:?}. It's RGB is {},{},{}",
///     my_color,
///     my_color.get_str("Red").unwrap(),
///     my_color.get_str("Green").unwrap(),
///     my_color.get_str("Blue").unwrap()
/// );
/// assert_eq!("My color is Red. It\'s RGB is 255,0,0", &display);
/// ```

#[cfg_attr(
    feature = "verbose-enumproperty-name",
    proc_macro_derive(StrumEnumProperty, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enumproperty-name"),
    proc_macro_derive(EnumProperty, attributes(strum))
)]
pub fn enum_properties(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks = macros::enum_properties::enum_properties_inner(&ast)
        .unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// Generate a new type with only the discriminant names.
///
/// Given an enum named `MyEnum`, generates another enum called `MyEnumDiscriminants` with the same variants, without any data fields.
/// This is useful when you wish to determine the variant of an enum from a String, but the variants contain any
/// non-`Default` fields. By default, the generated enum has the following derives:
/// `Clone, Copy, Debug, PartialEq, Eq`. You can add additional derives using the
/// `#[strum_discriminants(derive(AdditionalDerive))]` attribute.
///
/// ```
/// // Bring trait into scope
/// use std::str::FromStr;
/// use strum::IntoEnumIterator;
/// use strum_macros::{EnumDiscriminants, EnumIter, EnumString};
///
/// #[derive(Debug)]
/// struct NonDefault;
///
/// // simple example
/// # #[allow(dead_code)]
/// #[derive(Debug, EnumDiscriminants)]
/// #[strum_discriminants(derive(EnumString))]
/// enum MyEnum {
///     Variant0(NonDefault),
///     Variant1 { a: NonDefault },
/// }
///
/// // You can rename the generated enum using the `#[strum_discriminants(name(OtherName))]` attribute:
/// # #[allow(dead_code)]
/// #[derive(Debug, EnumDiscriminants)]
/// #[strum_discriminants(derive(EnumIter))]
/// #[strum_discriminants(name(MyVariants))]
/// enum MyEnumR {
///     Variant0(bool),
///     Variant1 { a: bool },
/// }
///
/// // test simple example
/// assert_eq!(
///     MyEnumDiscriminants::Variant0,
///     MyEnumDiscriminants::from_str("Variant0").unwrap()
/// );
/// // test rename example combined with EnumIter
/// assert_eq!(
///     vec![MyVariants::Variant0, MyVariants::Variant1],
///     MyVariants::iter().collect::<Vec<_>>()
/// );
/// ```
///
/// It is also possible to specify the visibility (e.g. `pub`/`pub(crate)`/etc.)
/// of the generated enum. By default, the generated enum inherits the
/// visibility of the parent enum it was generated from.
///
/// ```nocompile
/// use strum_macros::EnumDiscriminants;
///
/// // You can set the visibility of the generated enum using the `#[strum_discriminants(vis(..))]` attribute:
/// mod inner {
///     use strum_macros::EnumDiscriminants;
///
///     # #[allow(dead_code)]
///     #[derive(Debug, EnumDiscriminants)]
///     #[strum_discriminants(vis(pub))]
///     #[strum_discriminants(name(PubDiscriminants))]
///     enum PrivateEnum {
///         Variant0(bool),
///         Variant1 { a: bool },
///     }
/// }
///
/// // test visibility example, `PrivateEnum` should not be accessible here
/// assert_ne!(
///     inner::PubDiscriminants::Variant0,
///     inner::PubDiscriminants::Variant1,
/// );
/// ```
#[cfg_attr(
    feature = "verbose-enumdiscriminants-name",
    proc_macro_derive(StrumEnumDiscriminants, attributes(strum, strum_discriminants))
)]
#[cfg_attr(
    not(feature = "verbose-enumdiscriminants-name"),
    proc_macro_derive(EnumDiscriminants, attributes(strum, strum_discriminants))
)]
pub fn enum_discriminants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks = macros::enum_discriminants::enum_discriminants_inner(&ast)
        .unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

/// Add a constant `usize` equal to the number of variants.
///
/// For a given enum generates implementation of `strum::EnumCount`,
/// which adds a static property `COUNT` of type usize that holds the number of variants.
///
/// ```
/// use strum::{EnumCount, IntoEnumIterator};
/// use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
///
/// #[derive(Debug, EnumCountMacro, EnumIter)]
/// enum Week {
///     Sunday,
///     Monday,
///     Tuesday,
///     Wednesday,
///     Thursday,
///     Friday,
///     Saturday,
/// }
///
/// assert_eq!(7, Week::COUNT);
/// assert_eq!(Week::iter().count(), Week::COUNT);
///
/// ```
#[cfg_attr(
    feature = "verbose-enumcount-name",
    proc_macro_derive(StrumEnumCount, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enumcount-name"),
    proc_macro_derive(EnumCount, attributes(strum))
)]
pub fn enum_count(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let toks =
        macros::enum_count::enum_count_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}
