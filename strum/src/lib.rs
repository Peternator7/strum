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
//! strum = "0.23"
//! strum_macros = "0.23"
//!
//! # You can also access strum_macros exports directly through strum using the "derive" feature
//! strum = { version = "0.23", features = ["derive"] }
//! ```
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// only for documentation purposes
pub mod additional_attributes;

/// The ParseError enum is a collection of all the possible reasons
/// an enum can fail to parse from a string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ParseError {
    VariantNotFound,
}

#[cfg(feature = "std")]
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        // We could use our macro here, but this way we don't take a dependency on the
        // macros crate.
        match self {
            ParseError::VariantNotFound => write!(f, "Matching variant not found"),
        }
    }
}

#[cfg(feature = "std")]
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
/// # use std::fmt::Debug;
/// // You need to bring the type into scope to use it!!!
/// use strum::{EnumIter, IntoEnumIterator};
///
/// #[derive(EnumIter, Debug)]
/// enum Color {
///     Red,
///     Green { range: usize },
///     Blue(usize),
///     Yellow,
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
/// generic_iterator::<Color, _>(|color| println!("{:?}", color));
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
/// let my_pet = Pet::Dog;
/// assert_eq!("I have a dog", my_pet.get_message().unwrap());
/// ```
pub trait EnumMessage {
    fn get_message(&self) -> Option<&'static str>;
    fn get_detailed_message(&self) -> Option<&'static str>;
    fn get_serializations(&self) -> &'static [&'static str];
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
/// let history = Class::History;
/// assert_eq!("Ms.Frizzle", history.get_str("Teacher").unwrap());
/// ```
pub trait EnumProperty {
    fn get_str(&self, prop: &str) -> Option<&'static str>;
    fn get_int(&self, _prop: &str) -> Option<usize> {
        Option::None
    }

    fn get_bool(&self, _prop: &str) -> Option<bool> {
        Option::None
    }
}

/// A cheap reference-to-reference conversion. Used to convert a value to a
/// reference value with `'static` lifetime within generic code.
#[deprecated(
    since = "0.22.0",
    note = "please use `#[derive(IntoStaticStr)]` instead"
)]
pub trait AsStaticRef<T>
where
    T: ?Sized,
{
    fn as_static(&self) -> &'static T;
}

/// A trait for capturing the number of variants in Enum. This trait can be autoderived by
/// `strum_macros`.
pub trait EnumCount {
    const COUNT: usize;
}

/// A trait for retrieving the names of each variant in Enum. This trait can
/// be autoderived by `strum_macros`.
pub trait VariantNames {
    /// Names of the variants of this enum
    const VARIANTS: &'static [&'static str];
}

#[derive(Copy, Clone)]
pub struct OpaqueRepr<T: EnumRepr>(T::Repr, core::marker::PhantomData<T>);

pub trait EnumRepr {
    /// The repr type.
    type Repr: Copy
        + core::ops::BitOr
        + core::ops::BitOrAssign
        + num_traits::ops::wrapping::WrappingShl
        + num_traits::ops::wrapping::WrappingShl
        + num_traits::int::PrimInt;
    /// The opaque representation type.
    type OpaqueRepr;
    /// The enum type.
    type EnumT: EnumRepr;

    /// convert to the enums #[repr(..)]
    /// equivalent to `self as ..`
    fn to_repr(self) -> Self::Repr;
    /// Converts to a OpaqueRepr<Self>
    fn opaque(self) -> Self::OpaqueRepr;
    /// Non-const trait version of FromRepr
    fn cvt_from_repr(repr: Self::Repr) -> Option<Self::EnumT>;
}

impl<T: EnumRepr> core::ops::BitOr<T> for OpaqueRepr<T>
where
    Self: EnumRepr<Repr = <T as EnumRepr>::Repr>,
    <T as EnumRepr>::Repr: core::ops::BitOr<Output = <T as EnumRepr>::Repr>,
{
    type Output = Self;
    fn bitor(self, other: T) -> OpaqueRepr<T> {
        Self::from_repr(self.to_repr() | other.to_repr())
    }
}

impl<T: EnumRepr> core::ops::BitOr<Self> for OpaqueRepr<T>
where
    Self: EnumRepr<Repr = <T as EnumRepr>::Repr>,
    <T as EnumRepr>::Repr: core::ops::BitOr<Output = <T as EnumRepr>::Repr>,
{
    type Output = Self;
    fn bitor(self, other: OpaqueRepr<T>) -> OpaqueRepr<T> {
        Self::from_repr(self.to_repr() | other.to_repr())
    }
}

impl<T: EnumRepr<EnumT = T>> EnumRepr for OpaqueRepr<T> {
    type Repr = <T as EnumRepr>::Repr;
    type OpaqueRepr = Self;
    type EnumT = T;

    fn to_repr(self) -> Self::Repr {
        self.0
    }

    fn opaque(self) -> Self {
        self
    }

    fn cvt_from_repr(repr: Self::Repr) -> Option<Self::EnumT> {
        Self::EnumT::cvt_from_repr(repr)
    }
}

impl<T: EnumRepr> OpaqueRepr<T> {
    pub fn new(e: T) -> OpaqueRepr<T> {
        OpaqueRepr::<T>(e.to_repr(), core::marker::PhantomData)
    }

    fn from_repr(repr: T::Repr) -> OpaqueRepr<T> {
        OpaqueRepr::<T>(repr, core::marker::PhantomData)
    }
}

impl<
        R: num_traits::PrimInt
            + core::ops::BitOrAssign
            + num_traits::WrappingShr
            + num_traits::WrappingShl,
        E: EnumRepr<EnumT = E, Repr = R>,
        O: Clone + EnumRepr<EnumT = E, Repr = R>,
    > EnumMaskIter for O
{
    type I = EnumMaskIterator<R, E, O>;

    fn mask_iter(&mut self) -> EnumMaskIterator<R, E, O> {
        EnumMaskIterator {
            mask: self.clone().to_repr(),
            shift: 0,
            phantom: core::marker::PhantomData,
        }
    }
}

pub trait EnumMaskIter: Sized
where
    Self: EnumRepr,
    Self::Repr: core::ops::BitOr + core::ops::BitOrAssign,
    <Self as EnumRepr>::EnumT: EnumRepr,
{
    type I: Iterator<Item = Self::EnumT>;

    fn mask_iter(&mut self) -> Self::I;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumMaskIterator<
    R: num_traits::int::PrimInt + std::ops::BitOrAssign + num_traits::ops::wrapping::WrappingShl + num_traits::ops::wrapping::WrappingShr,
    T: EnumRepr<Repr = R, EnumT = T>,
    O: EnumRepr<Repr = R, EnumT = T>,
> {
    mask: R,
    shift: u32,
    phantom: core::marker::PhantomData<(O, R)>,
}

impl<
        R: num_traits::WrappingShr + num_traits::WrappingShl + num_traits::int::PrimInt + std::ops::BitOrAssign,
        T: EnumRepr<Repr = R, EnumT = T>,
        O: EnumRepr<Repr = R, EnumT = T>,
    > Iterator for EnumMaskIterator<R, T, O>
{
    type Item = <T as EnumRepr>::EnumT;

    fn next(&mut self) -> Option<Self::Item> {
        // This can doubtlessly be improved
        let tz: u32 = self.mask.trailing_zeros();
        let discr = ((self.mask.wrapping_shr(tz)) & num_traits::identities::one())
            .wrapping_shl(self.shift + tz);
        let one_u32: u32 = num_traits::identities::one();
        self.mask = self.mask.wrapping_shr(tz + one_u32);
        let shift_lhs: u32 = core::ops::Add::<u32>::add(tz, one_u32);
        self.shift += shift_lhs;
        T::EnumT::cvt_from_repr(discr)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // FIXME This assumes mask bits only contain enum repr items...
        // And there is only one bit to each enum discriminant.
        (
            self.mask.count_ones() as usize,
            Some(self.mask.count_ones() as usize),
        )
    }
}

// FIXME derive ExactSizeIterator...

#[cfg(feature = "derive")]
pub use strum_macros::*;

macro_rules! DocumentMacroRexports {
    ($($export:ident),+) => {
        $(
            #[cfg(all(docsrs, feature = "derive"))]
            #[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
            pub use strum_macros::$export;
        )+
    };
}

// We actually only re-export these items individually if we're building
// for docsrs. You can do a weird thing where you rename the macro
// and then reference it through strum. The renaming feature should be deprecated now that
// 2018 edition is almost 2 years old, but we'll need to give people some time to do that.
DocumentMacroRexports! {
    AsRefStr,
    AsStaticStr,
    Display,
    EnumCount,
    EnumDiscriminants,
    EnumIter,
    EnumMask,
    EnumMessage,
    EnumProperty,
    EnumString,
    EnumVariantNames,
    FromRepr,
    IntoStaticStr,
    ToString
}
