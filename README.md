# Strum

[![Build Status](https://travis-ci.com/Peternator7/strum.svg?branch=master)](https://travis-ci.com/Peternator7/strum)
[![Build status](https://ci.appveyor.com/api/projects/status/ji4f6n2m5lvu11xt?svg=true)](https://ci.appveyor.com/project/Peternator7/strum)
[![Latest Version](https://img.shields.io/crates/v/strum.svg)](https://crates.io/crates/strum)
[![Rust Documentation](https://docs.rs/strum/badge.svg)](https://docs.rs/strum)
![Crates.io](https://img.shields.io/crates/l/strum)
![Crates.io](https://img.shields.io/crates/d/strum)

Strum is a set of macros and traits for working with enums and strings easier in Rust.

# Compatibility

Strum is currently compatible with versions of rustc >= 1.32.0. Pull Requests that improve compatibility with older
versions are welcome. The project goal is to support a rust version for at least 2 years after release 
and even longer is preferred since this project changes slowly.

# Including Strum in Your Project

Import strum and strum_macros into your project by adding the following lines to your
Cargo.toml. Strum_macros contains the macros needed to derive all the traits in Strum.

```toml
[dependencies]
strum = "0.24"
strum_macros = "0.24"

# You can also use the "derive" feature, and import the macros directly from "strum"
# strum = { version = "0.24", features = ["derive"] }
```

# Strum Macros

Strum has implemented the following macros:

| Macro | Description |
| --- | ----------- |
| [EnumString] | Converts strings to enum variants based on their name. |
| [Display] | Converts enum variants to strings |
| [FromRepr] | Convert from an integer to an enum. |
| [AsRefStr] | Implement `AsRef<str>` for `MyEnum` |
| [IntoStaticStr] | Implements `From<MyEnum> for &'static str` on an enum |
| [EnumVariantNames] | Adds an associated `VARIANTS` constant which is an array of discriminant names |
| [EnumIter] | Creates a new type that iterates of the variants of an enum. |
| [EnumProperty] | Add custom properties to enum variants. |
| [EnumMessage] | Add a verbose message to an enum variant. |
| [EnumDiscriminants] | Generate a new type with only the discriminant names. |
| [EnumCount] | Add a constant `usize` equal to the number of variants. |

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
[EnumString]: https://docs.rs/strum_macros/0.24/strum_macros/derive.EnumString.html
[Display]: https://docs.rs/strum_macros/0.24/strum_macros/derive.Display.html
[AsRefStr]: https://docs.rs/strum_macros/0.24/strum_macros/derive.AsRefStr.html
[IntoStaticStr]: https://docs.rs/strum_macros/0.24/strum_macros/derive.IntoStaticStr.html
[EnumVariantNames]: https://docs.rs/strum_macros/0.24/strum_macros/derive.EnumVariantNames.html
[EnumIter]: https://docs.rs/strum_macros/0.24/strum_macros/derive.EnumIter.html
[EnumProperty]: https://docs.rs/strum_macros/0.24/strum_macros/derive.EnumProperty.html
[EnumMessage]: https://docs.rs/strum_macros/0.24/strum_macros/derive.EnumMessage.html
[EnumDiscriminants]: https://docs.rs/strum_macros/0.24/strum_macros/derive.EnumDiscriminants.html
[EnumCount]: https://docs.rs/strum_macros/0.24/strum_macros/derive.EnumCount.html
[FromRepr]: https://docs.rs/strum_macros/0.24/strum_macros/derive.FromRepr.html
