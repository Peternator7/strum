extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::IntoEnumIterator;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
#[strum_discriminants_derive(EnumIter)]
enum Simple {
    Variant0,
    Variant1,
}

#[test]
fn simple_test() {
    let discriminants = SimpleDiscriminants::iter().collect::<Vec<_>>();
    let expected = vec![SimpleDiscriminants::Variant0, SimpleDiscriminants::Variant1];

    assert_eq!(expected, discriminants);
}

#[derive(Debug)]
struct NonDefault;

#[allow(dead_code)]
#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants_derive(EnumIter)]
enum WithFields {
    Variant0(NonDefault),
    Variant1 { a: NonDefault },
}

#[test]
fn fields_test() {
    let discriminants = WithFieldsDiscriminants::iter().collect::<Vec<_>>();
    let expected = vec![
        WithFieldsDiscriminants::Variant0,
        WithFieldsDiscriminants::Variant1,
    ];

    assert_eq!(expected, discriminants);
}

trait Foo {}
trait Bar {}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
#[strum_discriminants_derive(EnumIter)]
enum Complicated<U: Foo, V: Bar> {
    /// With Docs
    A(U),
    B {
        v: V,
    },
    C,
}

#[test]
fn complicated_test() {
    let discriminants = ComplicatedDiscriminants::iter().collect::<Vec<_>>();
    let expected = vec![
        ComplicatedDiscriminants::A,
        ComplicatedDiscriminants::B,
        ComplicatedDiscriminants::C,
    ];

    assert_eq!(expected, discriminants);
}

// This test exists to ensure that we do not copy across the `#[strum(default = "true")]` meta
// attribute. If we do without deriving any `strum` derivations on the generated discriminant enum,
// Rust will generate a compiler error saying it doesn't understand the `strum` attribute.
#[allow(dead_code)]
#[derive(Debug, EnumDiscriminants)]
enum WithDefault {
    #[strum(default = "true")]
    A(String),
    B,
}

#[test]
fn with_default_test() {
    assert!(WithDefaultDiscriminants::A != WithDefaultDiscriminants::B);
}
