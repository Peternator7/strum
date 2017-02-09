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
//! #[derive(EnumString,EnumIter,EnumMessage)]
//! pub enum Color {
//!     Red,
//!     #[strum(serialize="b", serialize="blue", serialize="verboseblue")]
//!     Blue,
//!     #[strum(help="This is the color yellow")]
//!     Yellow
//! }
//! ```
//!
//! # Strum Macros
//!
//! Strum currently supports 3 procedural macros.
//!
//! 1. `EnumString`: EnumString derives `std::str::FromStr` on the enum. Let's look an example of how the code is generated to
//!     see what various attributes do to generated code.
//!
//!     ```dontrun
//!     #[derive(EnumString)]
//!     enum Color {
//!         Red,
//!         Green { range:usize }, // The Default value will be inserted into range.
//!         #[strum(serialize="blue",serialize="b")] // We can match on multiple different patterns.
//!         Blue(usize),
//!         #[strum(disabled="true")] // Notice that we can disable certain variants from being found.
//!         Yellow,
//!     }
//!
//!     // The generated code will look like:
//!     impl std::str::FromStr for Color {
//!         type Err = strum::ParseError;
//!
//!         fn from_str(s: &str) -> Result<#name #ty_generics,strum::ParseError> {
//!             match s {
//!                 "Red" => Ok(Color::Red),
//!                 "Green" => Ok(Green { range:Default::default(); }),
//!                 "blue" | "b" => Ok(Blue(Default::default())),
//!                 _ => Err(strum::ParseError::VariantNotFound),
//!             }
//!         }
//!     }
//!     ```
//!
//!     Notice how "Green" and "Blue" don't match on the associated data. That is intentional as parsing the input string
//!     cannot be done at compile time and the intention of this plugin is to design abstractions that are as cheap as
//!     possible at runtime. If you do need behavior like that, consider the more powerful Serde library for your serialization.
//!     See the explanation below on how to use `#[strum(default="true")]` to capture the input even if it doesn't match with
//!     any of the variants.
//!
//! 2. `EnumIter`: EnumIter let's you iterate over the variants of an Enum as long as 2 conditions are satisfied.
//!     The enum cannot have a lifetime bound on it, and any types on the variants must implement Default::default(). When
//!     EnumIter is derived, it creates a new type called `YourEnumIter` that is the actual iterator over your enum. Be aware
//!     of that to avoid potential naming collisions with this plugin. `EnumIter` implements the type `strum::IntoEnumIter` for
//!     your enum. This let's you write code like:
//!
//!     ```dontrun
//!     // You need to bring the type into scope to use it!!!
//!     use strum::IntoEnumIter;
//!
//!     #[derive(EnumIter,Debug)]
//!     enum Color {
//!         Red,
//!         Green { range:usize },
//!         Blue(usize),
//!         Yellow,
//!     }
//!
//!     // Generically print out all variants of an enum.
//!     // The 2nd bound is unpleasent looking, but can always be inferred.
//!     fn debug_enum<E,I:Iterator<Item=E>>() where E: IntoEnumIter<Iterator=I> {
//!         for e in E::iter() {
//!             println!("{:?}", e);
//!         }
//!     }
//!
//!     fn main() {
//!         debug_enum<Color,_>();
//!     }
//!     ```
//!
//!     Notice what you need to do if you want to generically iterate over any type of enum. It takes 2 type
//!     parameters to write the function, but you only ever need to know the first one.
//!
//! 3. `EnumMessage`: EnumMessage is designed to encode several static strings into the enum itself. This can be
//!     used for things like simpler error handling. This implements the trait `strum::EnumMessage` using the additional
//!     `#[strum(message="This is your message")]` attributes that you attach to each variant of your enum.
//!     You can also provided a `#[strum(detailed_message="Here is a detailed message")]` attribute to create a
//!     seperate more detailed message than the first. Here's what code will be generated for you:
//!
//!     ```dontrun
//!     // You need to bring the type into scope to use it!!!
//!     use strum::EnumMessage;
//!
//!     #[derive(EnumMessage,Debug)]
//!     enum Color {
//!         #[strum(message="Red",detailed_message="This is very red")]
//!         Red,
//!         #[strum(message="Simply Green")]
//!         Green { range:usize },
//!         #[strum(serialize="b",serialize="blue")]
//!         Blue(usize),
//!     }
//!
//!     // Generated code
//!     impl EnumMessage for Color {
//!         fn get_message(&self) -> Option<&str> {
//!             match self {
//!                 &Color::Red => Some("Red"),
//!                 &Color::Green => Some("Simply Green"),
//!                 _ => None
//!             }
//!         }
//!
//!         fn get_detailed_message(&self) -> Option<&str> {
//!             match self {
//!                 &Color::Red => Some("This is very red"),
//!                 &Color::Green => Some("Simply Green"),
//!                 _ => None
//!             }
//!         }
//!
//!         fn get_serializations(&self) -> Option<&str> {
//!             match self {
//!                 &Color::Red => {
//!                     static ARR: [&'static str; 1] = ["Red"];
//!                     &ARR
//!                 },
//!                 &Color::Green => {
//!                     static ARR: [&'static str; 1] = ["Green"];
//!                     &ARR
//!                 },
//!                 &Color::Blue => {
//!                     static ARR: [&'static str; 2] = ["b", "blue"];
//!                     &ARR
//!                 },
//!             }
//!         }
//!     }
//!     ```
//!
//! # Using Strum Attributes
//!
//! Strum supports several custom parameters to modify the code that is generated. All custom parameters can
//! be applied to any enum variants by using the #[strum(parameter="value")] attribute on variants.
//!
//! - `serialize`: Changes the text that `FromStr()` looks for when parsing a string. This attribute can
//!    be applied multiple times to an element and the enum variant will be parsed if any of them match.
//!
//! - `default`: Can be set on a single variant in an enum of the form `Variant(T)` where `T: From<&str>`.
//!    This tells the plugin when it's generating the code for `FromStr()` to generate
//!
//!     ```dontrun
//!     default => Ok(Variant(default.into()))
//!     ```
//!
//!     as the last line of the match statement instead of generating the usual code which is:
//!
//!     ```dontrun
//!     _ => Err(strum::ParseError::VariantNotFound)
//!     ```
//!
//!    In the former, whatever `&str` was passed in is wrapped into the variant. This will fail at compile time
//!    if the type inside the tuple variant doesn't implement `From<&str>`, but the error might be a bit hard to
//!    understand so keep that in mind. There can only be one `default` on an enum.
//!
//! - `disabled`: If this is "true" then this variant will be left out of generated code.
//!
//! - `message`: Allows adding a message to enum variants. This is used in conjunction with the `EnumMessages`
//!    trait to generate messages that can be displayed for each variant. If `detailed_message` is not provided,
//!    then `message` will also be returned when get_detailed_message() is called.
//!
//! - `detailed_message`: A more detailed message associated with a given variant. If this value is omitted, then
//!    `message` will be used in it's place.
//!
//! # Examples
//!
//! Using `EnumMessage` for quickly implementing `Error`
//!
//! ```dontrun
//! #[derive(Debug, EnumMessage)]
//! enum ServerError {
//!     #[strum(message="There was an error in the network connection")]
//!     NetworkError,
//!     #[strum(message="Could read the user input")]
//!     InvalidUserInputError,
//! }
//!
//! impl Error for ServerError {
//!     fn description(&self) -> &str {
//!         self.get_message().unwrap()
//!     }
//! }
//! ```
//!
//! Using `EnumString` to tokenize a series of inputs:
//!
//! ```dontrun
//! #[derive(Debug, EnumString)]
//! enum Tokens {
//!     #[strum(serialize="function")]
//!     Function,
//!     #[strum(serialize="("))]
//!     OpenParen
//!     #[strum(serialize=")"))]
//!     CloseParen
//!     #[strum(default="true")]
//!     Ident(String)
//! }
//!
//! fn main() {
//!     let toks = ["function", "hello_world", "(", ")"];
//!     for tok in &toks {
//!         println!("{:?}", Tokens::from_str(tok).unwrap());
//!     }
//!     // Tokens::Function, Tokens::Ident("hello_world"), Tokens::OpenParen, Tokens::CloseParen
//! }
//! ```

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
pub trait EnumMessage {
    fn get_message(&self) -> Option<&str>;
    fn get_detailed_message(&self) -> Option<&str>;
    fn get_serializations(&self) -> &[&str];
}
