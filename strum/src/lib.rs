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
//! ```rust
//! // Strum contains all the trait definitions
//! extern crate strum;
//! #[macro_use]
//! extern crate strum_macros;
//!
//! // Import the traits we use into scope.
//! use strum::{IntoEnumIterator, EnumMessage};
//!
//! #[derive(EnumString,EnumIter,EnumMessage)]
//! pub enum Color {
//!     Red,
//!     #[strum(serialize="b", serialize="blue", serialize="verboseblue")]
//!     Blue,
//!     #[strum(message="This is the color yellow")]
//!     Yellow
//! }
//! # fn main() {}
//! ```
//!
//! # Strum Macros
//!
//! Strum currently supports 3 procedural macros.
//!
//! 1. `EnumString`: EnumString derives `std::str::FromStr` on the enum. Let's look an example of how the code is generated to
//!     see what various attributes do to generated code.
//!
//!     ```
//!     # extern crate strum;
//!     # #[macro_use] extern crate strum_macros;
//!     #[derive(EnumString)]
//!     enum Color {
//!         Red,
//!
//!         // The Default value will be inserted into range if we match "Green".
//!         Green { range:usize },
//!
//!         // We can match on multiple different patterns.
//!         #[strum(serialize="blue",serialize="b")]
//!         Blue(usize),
//!
//!         // Notice that we can disable certain variants from being found
//!         #[strum(disabled="true")]
//!         Yellow,
//!     }
//!
//!     /*
//!     //The generated code will look like:
//!     impl std::str::FromStr for Color {
//!         type Err = strum::ParseError;
//!
//!         fn from_str(s: &str) -> Result<Color, strum::ParseError> {
//!             match s {
//!                 "Red" => Ok(Color::Red),
//!                 "Green" => Ok(Color::Green { range:Default::default() }),
//!                 "blue" | "b" => Ok(Color::Blue(Default::default())),
//!                 _ => Err(strum::ParseError::VariantNotFound),
//!             }
//!         }
//!     }
//!     */
//!     # fn main() {}
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
//!     ```rust
//!     # extern crate strum;
//!     # #[macro_use] extern crate strum_macros;
//!     # use std::fmt::Debug;
//!     // You need to bring the type into scope to use it!!!
//!     use strum::IntoEnumIterator;
//!
//!     #[derive(EnumIter,Debug)]
//!     enum Color {
//!         Red,
//!         Green { range:usize },
//!         Blue(usize),
//!         Yellow,
//!     }
//!
//!     // It's simple to iterate over the variants of an enum.
//!     fn simple_example() {
//!         for color in Color::iter() {
//!             println!("My favorite color is {:?}", color);
//!         }
//!     }
//!
//!     // Iterating over any enum requires 2 type parameters
//!     // A 3rd is used in this example to allow passing a predicate
//!     fn generic_iterator<E, I, F>(pred: F)
//!                         where E: IntoEnumIterator<Iterator=I>,
//!                               I: Iterator<Item=E>,
//!                               F: Fn(E) {
//!         for e in E::iter() {
//!             pred(e)
//!         }
//!     }
//!
//!     fn main() {
//!         simple_example();
//!         generic_iterator::<Color,_, _>(|color| println!("{:?}", color));
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
//!     ```rust
//!     # extern crate strum;
//!     # #[macro_use] extern crate strum_macros;
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
//!     /*
//!     // Generated code
//!     impl EnumMessage for Color {
//!         fn get_message(&self) -> Option<&str> {
//!             match self {
//!                 &Color::Red => Some("Red"),
//!                 &Color::Green {..} => Some("Simply Green"),
//!                 _ => None
//!             }
//!         }
//!
//!         fn get_detailed_message(&self) -> Option<&str> {
//!             match self {
//!                 &Color::Red => Some("This is very red"),
//!                 &Color::Green {..}=> Some("Simply Green"),
//!                 _ => None
//!             }
//!         }
//!
//!         fn get_serializations(&self) -> &[&str] {
//!             match self {
//!                 &Color::Red => {
//!                     static ARR: [&'static str; 1] = ["Red"];
//!                     &ARR
//!                 },
//!                 &Color::Green {..}=> {
//!                     static ARR: [&'static str; 1] = ["Green"];
//!                     &ARR
//!                 },
//!                 &Color::Blue (..) => {
//!                     static ARR: [&'static str; 2] = ["b", "blue"];
//!                     &ARR
//!                 },
//!             }
//!         }
//!     }
//!     */
//!     # fn main() {}
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
//!     ```ignore
//!     default => Ok(Variant(default.into()))
//!     ```
//!
//!     as the last line of the match statement instead of generating the usual code which is:
//!
//!     ```ignore
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
//! ```rust
//! extern crate strum;
//! #[macro_use]
//! extern crate strum_macros;
//! # use std::error::Error;
//! # use std::fmt::*;
//! use strum::EnumMessage;
//!
//! #[derive(Debug, EnumMessage)]
//! enum ServerError {
//!     #[strum(message="A network error occured")]
//!     #[strum(detailed_message="Try checking your connection.")]
//!     NetworkError,
//!     #[strum(message="User input error.")]
//!     #[strum(detailed_message="There was an error parsing user input. Please try again.")]
//!     InvalidUserInputError,
//! }
//!
//! impl Display for ServerError {
//!     fn fmt(&self, f: &mut Formatter) -> Result {
//!         write!(f, "{}", self.get_message().unwrap())
//!     }
//! }
//!
//! impl Error for ServerError {
//!     fn description(&self) -> &str {
//!         self.get_detailed_message().unwrap()
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! Using `EnumString` to tokenize a series of inputs:
//!
//! ```rust
//! extern crate strum;
//! #[macro_use]
//! extern crate strum_macros;
//! use std::str::FromStr;
//!
//! #[derive(Debug, EnumString)]
//! enum Tokens {
//!     #[strum(serialize="function")]
//!     Function,
//!     #[strum(serialize="(")]
//!     OpenParen,
//!     #[strum(serialize=")")]
//!     CloseParen,
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
#[derive(Debug,Clone,Copy,Eq,PartialEq,Hash)]
pub enum ParseError {
    VariantNotFound,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
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
