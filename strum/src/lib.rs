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
//! strum = "0.18.0"
//! strum_macros = "0.18.0"
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
//! | [EnumString] | Converts strings to enum variants based on their name |
//! | [Display] | Converts enum variants to strings |
//! | [AsRefStr] | Converts enum variants to `&'static str` |
//! | [IntoStaticStr] | Implements `From<MyEnum> for &'static str` on an enum |
//! | [EnumVariantNames] | Implements Strum::VariantNames which adds an associated constant `VARIANTS` which is an array of discriminant names |
//! | [EnumIter] | Creates a new type that iterates of the variants of an enum. |
//! | [EnumProperty] | Add custom properties to enum variants. |
//! | [EnumMessage] | Add a verbose message to an enum variant. |
//! | [EnumDiscriminants] | Generate a new type with only the discriminant names. |
//! | [EnumCount] | Add a constant `usize` equal to the number of variants. |
//!
//! [EnumString]: https://github.com/Peternator7/strum/wiki/Derive-EnumString
//! [Display]: https://github.com/Peternator7/strum/wiki/Derive-Display
//! [AsRefStr]: https://github.com/Peternator7/strum/wiki/Derive-AsRefStr
//! [IntoStaticStr]: https://github.com/Peternator7/strum/wiki/Derive-IntoStaticStr
//! [EnumVariantNames]: https://github.com/Peternator7/strum/wiki/Derive-EnumVariantNames
//! [EnumIter]: https://github.com/Peternator7/strum/wiki/Derive-EnumIter
//! [EnumProperty]: https://github.com/Peternator7/strum/wiki/Derive-EnumProperty
//! [EnumMessage]: https://github.com/Peternator7/strum/wiki/Derive-EnumMessage
//! [EnumDiscriminants]: https://github.com/Peternator7/strum/wiki/Derive-EnumDiscriminants
//! [EnumCount]: https://github.com/Peternator7/strum/wiki/Derive-EnumCount

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
            ParseError::VariantNotFound => write!(f, "Matching variant not found"),
        }
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        match self {
            ParseError::VariantNotFound => {
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
/// // Iterate over the items in an enum and perform some function on them.
/// fn generic_iterator<E, F>(pred: F)
/// where
///     E: IntoEnumIterator,
///     F: Fn(E),
/// {
///     for e in E::iter() {
///         pred(e)
///     }
/// }
///
/// fn main() {
///     generic_iterator::<Color, _>(|color| println!("{:?}", color));
/// }
/// ```
pub trait IntoEnumIterator: Sized {
    type Iterator: Iterator<Item = Self>;

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

/// A trait for retrieving the names of each variant in Enum. This trait can
/// be autoderived by `strum_macros`.
pub trait VariantNames {
    /// Names of the variants of this enum
    const VARIANTS: &'static [&'static str];
}

#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate strum_macros;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use strum_macros::*;
