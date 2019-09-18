# Strum

[![Build Status](https://travis-ci.org/Peternator7/strum.svg?branch=master)](https://travis-ci.org/Peternator7/strum)
[![Latest Version](https://img.shields.io/crates/v/strum.svg)](https://crates.io/crates/strum)
[![Rust Documentation](https://docs.rs/strum/badge.svg)](https://docs.rs/strum)

Strum is a set of macros and traits for working with enums and strings easier in Rust.

# Compatibility

Strum is compatible with versions of rustc >= 1.31.0. That's the earliest version of stable rust that supports
impl trait. Pull Requests that improve compatibility with older versions are welcome, but new feature work
will focus on the current version of rust with an effort to avoid breaking compatibility with older versions.

# Including Strum in Your Project

Import strum and strum_macros into your project by adding the following lines to your
Cargo.toml. Strum_macros contains the macros needed to derive all the traits in Strum.

```toml
[dependencies]
strum = "0.16.0"
strum_macros = "0.16.0"
```

And add these lines to the root of your project, either lib.rs or main.rs.

```rust
// Strum contains all the trait definitions
extern crate strum;
#[macro_use]
extern crate strum_macros;

// Instead of #[macro_use], newer versions of rust should prefer
use strum_macros::{Display, EnumIter}; // etc.
```

# Strum Macros

Strum has implemented the following macros:

| Macro | Description |
| --- | ----------- |
| [EnumString] | Converts strings to enum variants based on their name |
| [Display] | Converts enum variants to strings |
| [AsRefStr] | Converts enum variants to `&'static str` |
| [IntoStaticStr] | Implements `From<MyEnum> for &'static str` on an enum |
| [EnumVariantNames] | Adds a `variants` method returning an array of discriminant names |
| [EnumIter] | Creates a new type that iterates of the variants of an enum. |
| [EnumProperty] | Add custom properties to enum variants. |
| [EnumMessage] | Add a verbose message to an enum variant. |
| [EnumDiscriminants] | Generate a new type with only the discriminant names. |
| [EnumCount] | Add a constant `usize` equal to the number of variantes. |

# Contributing

Thanks for your interest in contributing. The project is divided into 3 parts, the traits are in the
`/strum` folder. The procedural macros are in the `/strum_macros` folder, and the integration tests are
in `/strum_tests`. If you are adding additional features to `strum` or `strum_macros`, you should make sure
to run the tests and add new integration tests to make sure the features work as expected.

# Debugging

To see the generated code, set the STRUM_DEBUG environment variable before compiling your code.
`STRUM_DEBUG=1` will dump all of the generated code for every type. `STRUM_DEBUG=YourType` will
only dump the code generated on a type named `YourType`.

# Name

Strum is short for STRing enUM because it's a library for augmenting enums with additional
information through strings.

Strumming is also a very whimsical motion, much like writing Rust code.

[Macro-Renames]: https://github.com/Peternator7/strum/wiki/Macro-Renames
[EnumString]: https://github.com/Peternator7/strum/wiki/Derive-EnumString
[Display]: https://github.com/Peternator7/strum/wiki/Derive-Display
[AsRefStr]: https://github.com/Peternator7/strum/wiki/Derive-AsRefStr
[IntoStaticStr]: https://github.com/Peternator7/strum/wiki/Derive-IntoStaticStr
[EnumVariantNames]: https://github.com/Peternator7/strum/wiki/Derive-EnumVariantNames
[EnumIter]: https://github.com/Peternator7/strum/wiki/Derive-EnumIter
[EnumProperty]: https://github.com/Peternator7/strum/wiki/Derive-EnumProperty
[EnumMessage]: https://github.com/Peternator7/strum/wiki/Derive-EnumMessage
[EnumDiscriminants]: https://github.com/Peternator7/strum/wiki/Derive-EnumDiscriminants
[EnumCount]: https://github.com/Peternator7/strum/wiki/Derive-EnumCount
