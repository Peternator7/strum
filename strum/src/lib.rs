//! # Strum
//!
//! Strum = STRing enUM. It's a small set of macros and traits for working with and parsing enums.
//! One possible use case is parsing command line args into strongly type enums, and the power of
//! Strum comes from the simplicity of the code it generates. It generates the code you would
//! write by hand automatically.
//!
//! # Importing Strum macros
//!
//! Strum uses Macros 1.1 to implement Custom Derives for enums. To include strum_macros and strum
//! in your project, add these lines to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! strum = "*"
//! strum_macros = "*"
//! ```
//!
//! And you need to add these to the root of your project, either lib.rs or main.rs.
//!
//! ```dontrun
//! // Strum contains all the trait definitions
//! extern crate strum;
//! #[macro_use]
//! extern crate strum_macros;
//!
//! // Import the traits we use into scope.
//! use strum::*;
//!
//! #[derive(EnumString,EnumIter,EnumHelp)]
//! pub enum Color {
//!     Red,
//!     #[strum(serialize="b", serialize="blue", serialize="verboseblue")]
//!     Blue,
//!     #[strum(help="This is the color yellow")]
//!     Yellow
//! }
//! ```
//!
//! # Using Strum Attributes
//!
//! Strum supports several custom parameters to modify the code that is generated. All custom parameters can
//! be applied to any enum variants by using the #[strum(parameter="")] attribute on variants.
//!
//! - `serialize`: Changes the text that `FromStr()` looks for when parsing a string. This attribute can
//!    be applied multiple times to an element and the enum variant will be parsed if any of them match.
//!
//! - `disabled`: If this is "true" then this variant will be left out of generated code.
//!
//! - `message`: Allows adding a message to enum variants. This is used in conjunction with the `EnumMessages`
//!    trait to generate messages that can be displayed for each variant. This is potentially useful for a
//!    situation like parsing command line arguments and displaying a help message based on what the user
//!    entered.
//!


/// The ParseError enum is a collection of all the possible reasons
/// an enum can fail to parse from a string.
pub enum ParseError {
    VariantNotFound,
}

/// This trait designates that an `Enum` can be iterated over. It can
/// be auto generated using `strum_macros` on your behalf. The marker
/// trait let's you program generically over any enum that can be
/// iterated over.
pub trait IntoEnumIterator {
    type Iterator;

    fn iter() -> Self::Iterator;
}

/// EnumMessages can be auto implemented by `strum_macros`. This trait is designed
/// to allow associating a piece of text with specific variants automatically.
pub trait EnumMessages {
    fn get_message(&self) -> &str;
    fn get_detailed_message(&self) -> &str;
}
