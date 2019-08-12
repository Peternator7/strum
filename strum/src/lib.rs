//! # Strum
//!
//! [![Build Status](https://travis-ci.org/Peternator7/strum.svg?branch=master)](https://travis-ci.org/Peternator7/strum)
//! [![Latest Version](https://img.shields.io/crates/v/strum.svg)](https://crates.io/crates/strum)
//! [![Rust Documentation](https://docs.rs/strum/badge.svg)](https://docs.rs/strum)
//!
//! Strum is a set of macros and traits for working with
//! enums and strings easier in Rust.
//!
//! The full version of the README can be found on [Github](https://github.com/Peternator7/strum).
//!
//! # Including Strum in Your Project
//!
//! Import strum and strum_macros into your project by adding the following lines to your
//! Cargo.toml. Strum_macros contains the macros needed to derive all the traits in Strum.
//!
//! ```toml
//! [dependencies]
//! strum = "0.15.0"
//! strum_macros = "0.15.0"
//! ```
//!
//! And add these lines to the root of your project, either lib.rs or main.rs.
//!
//! ```rust
//! // Strum contains all the trait definitions
//! extern crate strum;
//! #[macro_use]
//! extern crate strum_macros;
//! # fn main() {}
//! ```
//!
//! # Strum Macros
//!
//! Strum has implemented the following macros:
//!
//! | Macro | Description |
//! | --- | ----------- |
//! | [EnumString](https://github.com/Peternator7/strum#EnumString) | Converts strings to enum variants based on their name |
//! | [Display](https://github.com/Peternator7/strum#Display) | Converts enum variants to strings |
//! | [AsRefStr](https://github.com/Peternator7/strum#AsRefStr) | Converts enum variants to `&'static str` |
//! | [IntoStaticStr](https://github.com/Peternator7/strum#IntoStaticStr) | Implements `From<MyEnum> for &'static str` on an enum |
//! | [EnumIter](https://github.com/Peternator7/strum#EnumIter) | Creates a new type that iterates of the variants of an enum. |
//! | [EnumProperty](https://github.com/Peternator7/strum#EnumProperty) | Add custom properties to enum variants. |
//! | [EnumMessage](https://github.com/Peternator7/strum#EnumMessage) | Add a verbose message to an enum variant. |
//! | [EnumDiscriminants](https://github.com/Peternator7/strum#EnumDiscriminants) | Generate a new type with only the discriminant names. |
//! | [EnumCount](https://github.com/Peternator7/strum#EnumCount) | Add a constant `usize` equal to the number of variantes. |
//!

/// The ParseError enum is a collection of all the possible reasons
/// an enum can fail to parse from a string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ParseError {
    VariantNotFound,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        // We could use our macro here, but this way we don't take a dependency on the
        // macros crate.
        match self {
            &ParseError::VariantNotFound => write!(f, "Matching variant not found"),
        }
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        match self {
            &ParseError::VariantNotFound => {
                "Unable to find a variant of the given enum matching the string given. Matching \
                 can be extended with the Serialize attribute and is case sensitive."
            }
        }
    }
}

/// This trait designates that an `Enum` can be iterated over. It can
/// be auto generated using `strum_macros` on your behalf.
///
/// # Example
///
/// ```rust
/// # extern crate strum;
/// # #[macro_use] extern crate strum_macros;
/// # use std::fmt::Debug;
/// // You need to bring the type into scope to use it!!!
/// use strum::IntoEnumIterator;
///
/// #[derive(EnumIter,Debug)]
/// enum Color {
///         Red,
///         Green { range:usize },
///         Blue(usize),
///         Yellow,
/// }
///
/// // Iterating over any enum requires 2 type parameters
/// // A 3rd is used in this example to allow passing a predicate
/// fn generic_iterator<E, I, F>(pred: F)
///                      where E: IntoEnumIterator<Iterator=I>,
///                            I: Iterator<Item=E>,
///                            F: Fn(E) {
///     for e in E::iter() {
///         pred(e)
///     }
/// }
///
/// fn main() {
///     generic_iterator::<Color,_, _>(|color| println!("{:?}", color));
/// }
/// ```
pub trait IntoEnumIterator {
    type Iterator;

    fn iter() -> Self::Iterator;
}

/// Associates additional pieces of information with an Enum. This can be
/// autoimplemented by deriving `EnumMessage` and annotating your variants with
/// `#[strum(message="...")].
///
/// # Example
///
/// ```rust
/// # extern crate strum;
/// # #[macro_use] extern crate strum_macros;
/// # use std::fmt::Debug;
/// // You need to bring the type into scope to use it!!!
/// use strum::EnumMessage;
///
/// #[derive(PartialEq, Eq, Debug, EnumMessage)]
/// enum Pet {
///     #[strum(message="I have a dog")]
///     #[strum(detailed_message="My dog's name is Spots")]
///     Dog,
///     #[strum(message="I don't have a cat")]
///     Cat,
/// }
///
/// fn main() {
///     let my_pet = Pet::Dog;
///     assert_eq!("I have a dog", my_pet.get_message().unwrap());
/// }
/// ```
pub trait EnumMessage {
    fn get_message(&self) -> Option<&str>;
    fn get_detailed_message(&self) -> Option<&str>;
    fn get_serializations(&self) -> &[&str];
}

/// EnumProperty is a trait that makes it possible to store additional information
/// with enum variants. This trait is designed to be used with the macro of the same
/// name in the `strum_macros` crate. Currently, the only string literals are supported
/// in attributes, the other methods will be implemented as additional attribute types
/// become stabilized.
///
/// # Example
///
/// ```rust
/// # extern crate strum;
/// # #[macro_use] extern crate strum_macros;
/// # use std::fmt::Debug;
/// // You need to bring the type into scope to use it!!!
/// use strum::EnumProperty;
///
/// #[derive(PartialEq, Eq, Debug, EnumProperty)]
/// enum Class {
///     #[strum(props(Teacher="Ms.Frizzle", Room="201"))]
///     History,
///     #[strum(props(Teacher="Mr.Smith"))]
///     #[strum(props(Room="103"))]
///     Mathematics,
///     #[strum(props(Time="2:30"))]
///     Science,
/// }
///
/// fn main() {
///     let history = Class::History;
///     assert_eq!("Ms.Frizzle", history.get_str("Teacher").unwrap());
/// }
/// ```
pub trait EnumProperty {
    fn get_str(&self, &str) -> Option<&'static str>;
    fn get_int(&self, &str) -> Option<usize> {
        Option::None
    }

    fn get_bool(&self, &str) -> Option<bool> {
        Option::None
    }
}

/// A cheap reference-to-reference conversion. Used to convert a value to a
/// reference value with `'static` lifetime within generic code.
/// #[deprecated(since="0.13.0", note="please use `#[derive(IntoStaticStr)]` instead")]
pub trait AsStaticRef<T>
where
    T: ?Sized,
{
    fn as_static(&self) -> &'static T;
}

/// A trait for capturing the number of variants in Enum. This trait can be autoderived by
/// `strum_macros`.
pub trait EnumCount {
    fn count() -> usize;
}

#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate strum_macros;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use strum_macros::*;
