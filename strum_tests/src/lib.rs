use strum::{Display, EnumCount, EnumDiscriminants, EnumString};
use strum_macros::EnumIs;

#[derive(Debug, Eq, PartialEq, EnumString, Display, EnumCount, EnumDiscriminants, EnumIs)]
pub enum Color {
    /// Docs on red
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(disabled)]
    Green(String),
    #[strum(default)]
    Purple(String),
}

/// A bunch of errors
///
/// This will not work:
///
/// ```compile_fail
/// # use strum_tests::{Errors, ErrorsDiscriminants};
/// fn expect_path_error(error: &Errors) {
///     let discriminant: ErrorsDiscriminants = error.into();
///     match discriminant {
///         ErrorsDiscriminants::PathError => (),
///         ErrorsDiscriminants::NotFound => panic!("should be a path error"),
///     }
/// }
/// ```
///
/// This will work:
///
/// ```
/// # use strum_tests::{Errors, ErrorsDiscriminants};
/// fn expect_path_error(error: &Errors) {
///     let discriminant: ErrorsDiscriminants = error.into();
///     match discriminant {
///         ErrorsDiscriminants::PathError => (),
///         ErrorsDiscriminants::NotFound => panic!("should be a path error"),
///         _ => panic!("unknown error, should be a path error"),
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, EnumDiscriminants)]
#[non_exhaustive]
#[strum_discriminants(doc = "Discriminants-only version of a bunch of errors")]
#[strum_discriminants(non_exhaustive)]
pub enum Errors {
    NotFound,
    PathError(String),
}
