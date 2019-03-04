extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::IntoEnumIterator;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
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
#[strum_discriminants(derive(EnumIter))]
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
#[strum_discriminants(derive(EnumIter))]
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

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
#[strum_discriminants(name(EnumBoo), derive(EnumIter))]
enum Renamed {
    Variant0(bool),
    Variant1(i32),
}

#[test]
fn renamed_test() {
    let discriminants = EnumBoo::iter().collect::<Vec<_>>();
    let expected = vec![EnumBoo::Variant0, EnumBoo::Variant1];

    assert_eq!(expected, discriminants);
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
#[strum_discriminants(name(SplitAttributesBoo), derive(Display))]
#[strum_discriminants(derive(EnumIter))]
enum SplitAttributes {
    Variant0(bool),
    Variant1(i32),
}

#[test]
fn split_attributes_test() {
    let discriminants = SplitAttributesBoo::iter().collect::<Vec<_>>();
    let expected = vec![SplitAttributesBoo::Variant0, SplitAttributesBoo::Variant1];

    assert_eq!(expected, discriminants);
    assert_eq!("Variant0", format!("{}", SplitAttributesBoo::Variant0));
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
#[strum_discriminants(
    name(PassThroughBoo),
    derive(Display, EnumIter, EnumString),
    strum(serialize_all = "snake_case")
)]
enum PassThrough {
    DarkBlack(bool),
    BrightWhite(i32),
}

#[test]
fn arbitrary_attributes_pass_through() {
    use std::str::FromStr;

    let discriminants = PassThroughBoo::iter().collect::<Vec<_>>();
    let expected = vec![PassThroughBoo::DarkBlack, PassThroughBoo::BrightWhite];

    assert_eq!(expected, discriminants);
    assert_eq!("dark_black", PassThroughBoo::DarkBlack.to_string());
    assert_eq!(
        PassThroughBoo::DarkBlack,
        PassThroughBoo::from_str("dark_black").unwrap()
    );
}

#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
enum EnumInto {
    A(bool),
    B(i32),
}

#[test]
fn from_test() {
    assert_eq!(EnumIntoDiscriminants::A, EnumInto::A(true).into());
    assert_eq!(EnumIntoDiscriminants::B, EnumInto::B(1).into());
}

#[test]
fn from_ref_test() {
    assert_eq!(EnumIntoDiscriminants::A, (&EnumInto::A(true)).into());
    assert_eq!(EnumIntoDiscriminants::B, (&EnumInto::B(1)).into());
}

#[derive(Debug)]
struct Rara;

#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
#[strum_discriminants(name(EnumIntoComplexVars))]
enum EnumIntoComplex<'a, T: 'a> {
    A(&'a T),
}

#[test]
fn from_test_complex() {
    let rara = Rara;
    assert_eq!(EnumIntoComplexVars::A, EnumIntoComplex::A(&rara).into());
}

#[test]
fn from_ref_test_complex() {
    let rara = Rara;
    assert_eq!(EnumIntoComplexVars::A, (&EnumIntoComplex::A(&rara)).into());
}
