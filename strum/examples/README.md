# Example uses for strum

## EnumString

auto-derives `std::str::FromStr` on the enum. Each variant of the enum will match on it's own name.
This can be overridden using `serialize="DifferentName"` or `to_string="DifferentName"`
on the attribute as shown below.
Multiple deserializations can be added to the same variant. If the variant contains additional data,
they will be set to their default values upon deserialization.

The `default` attribute can be applied to a tuple variant with a single data parameter. When a match isn't
found, the given variant will be returned and the input string will be captured in the parameter.

Note that the implementation of `FromStr` by default only matches on the name of the
variant. There is an option to match on different case conversions through the
`#[strum(serialize_all = "snake_case")]` type attribute. See the [Additional Attributes](https://github.com/Peternator7/strum/wiki/Additional-Attributes)
Section for more information on using this feature.

`cargo run --example enumstring`

## Display

Deriving `Display` on an enum prints out the given enum. This enables you to perform round trip style conversions
from enum into string and back again for unit style variants.
`Display` choose which serialization to used based on the following criteria:

1. If there is a `to_string` property, this value will be used. There can only be one per variant.
2. Of the various `serialize` properties, the value with the longest length is chosen. If that
    behavior isn't desired, you should use `to_string`.
3. The name of the variant will be used if there are no `serialize` or `to_string` attributes.

`cargo run --example display`

## AsRefStr

Implements `AsRef<str>` on your enum using the same rules as
`Display` for determining what string is returned. The difference is that `as_ref()` returns
a `&str` instead of a `String` so you don't allocate any additional memory with each call.

`cargo run --example asrefstr`

## IntoStaticStr

Implements `From<YourEnum>` and `From<&'a YourEnum>` for `&'static str`. This is
useful for turning an enum variant into a static string.
The Rust `std` provides a blanket impl of the reverse direction - i.e. `impl Into<&'static str> for YourEnum`.

`cargo run --example intostaticstr`

## EnumVariantNames

Adds an `impl` block for the `enum` that adds a static `VARIANTS` array of `&'static str` that are the discriminant names.
This will respect the `serialize_all` attribute on the `enum` (like `#[strum(serialize_all = "snake_case")]`, see **Additional Attributes** in the examples source code).

This example shows how to use this macro with structopt, you can run it using e.g.:

`cargo run --example enumvariantnames -- --color blue`

## EnumIter

Iterate over the variants of an Enum. Any additional data on your variants will be set to `Default::default()`.
The macro implements `strum::IntoEnumIter` on your enum and creates a new type called `YourEnumIter` that is the iterator object.
You cannot derive `EnumIter` on any type with a lifetime bound (`<'a>`) because the iterator would surely
create [unbounded lifetimes](https://doc.rust-lang.org/nightly/nomicon/unbounded-lifetimes.html).

`cargo run --example enumiter`
