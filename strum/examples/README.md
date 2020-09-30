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

## EnumVariantNames

Adds an `impl` block for the `enum` that adds a static `VARIANTS` array of `&'static str` that are the discriminant names.
This will respect the `serialize_all` attribute on the `enum` (like `#[strum(serialize_all = "snake_case")]`, see **Additional Attributes** in the examples source code).

This example shows how to use this macro with structopt, you can run it using e.g.:

`cargo run --example enumvariantnames -- --color blue`

