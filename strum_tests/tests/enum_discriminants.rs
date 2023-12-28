use std::mem::{align_of, size_of};

use enum_variant_type::EnumVariantType;
use strum::{
    Display, EnumDiscriminants, EnumIter, EnumMessage, EnumString, FromRepr, IntoEnumIterator,
};

mod core {} // ensure macros call `::core`

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

// This test exists to ensure that we can pass attributes to the discriminant variants.
#[allow(dead_code)]
#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumMessage))]
enum WithPassthroughAttributes {
    #[strum_discriminants(strum(message = "AAA"))]
    A(String),
    B,
}

#[test]
fn with_passthrough_attributes_test() {
    assert_eq!(
        WithPassthroughAttributesDiscriminants::A.get_message(),
        Some("AAA")
    );
    assert_eq!(
        WithPassthroughAttributesDiscriminants::B.get_message(),
        None
    );
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
    assert_eq!(EnumIntoDiscriminants::A, (EnumInto::A(true)).into());
    assert_eq!(EnumIntoDiscriminants::B, (EnumInto::B(1)).into());
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
    assert_eq!(EnumIntoComplexVars::A, (EnumIntoComplex::A(&rara)).into());
}

#[allow(dead_code)]
#[rustversion::since(1.34)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants, EnumVariantType)]
#[strum_discriminants(
    name(VariantFilterAttrDiscs),
    derive(Display, EnumIter, EnumString),
    strum(serialize_all = "snake_case")
)]
enum VariantFilterAttr {
    #[evt(derive(Clone, Copy, Debug))]
    DarkBlack(bool),
    #[evt(skip)]
    BrightWhite(i32),
}

#[rustversion::attr(since(1.34), test)]
#[rustversion::since(1.34)]
fn filter_variant_attributes_pass_through() {
    use std::str::FromStr;

    let discriminants = VariantFilterAttrDiscs::iter().collect::<Vec<_>>();
    let expected = vec![
        VariantFilterAttrDiscs::DarkBlack,
        VariantFilterAttrDiscs::BrightWhite,
    ];

    assert_eq!(expected, discriminants);
    assert_eq!("dark_black", VariantFilterAttrDiscs::DarkBlack.to_string());
    assert_eq!(
        VariantFilterAttrDiscs::DarkBlack,
        VariantFilterAttrDiscs::from_str("dark_black").unwrap()
    );
}

#[test]
#[rustversion::since(1.34)]
fn override_visibility() {
    mod private {
        use super::*;

        #[allow(dead_code)]
        #[derive(EnumDiscriminants)]
        #[strum_discriminants(name(PubDiscriminants), vis(pub))]
        enum PrivateEnum {
            VariantA(bool),
            VariantB(bool),
        }
    }

    assert_ne!(
        private::PubDiscriminants::VariantA,
        private::PubDiscriminants::VariantB,
    );
}

#[test]
#[rustversion::before(1.34)]
fn override_visibility() {
    mod private {
        use super::*;

        #[allow(dead_code)]
        #[derive(EnumDiscriminants)]
        #[strum_discriminants(name(PubDiscriminants), vis(r#pub))]
        enum PrivateEnum {
            VariantA(bool),
            VariantB(bool),
        }
    }

    assert_ne!(
        private::PubDiscriminants::VariantA,
        private::PubDiscriminants::VariantB,
    );
}

#[test]
fn crate_module_path_test() {
    pub mod nested {
        pub mod module {
            pub use strum;
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
    #[strum_discriminants(derive(EnumIter))]
    #[strum(crate = "nested::module::strum")]
    enum Simple {
        Variant0,
        Variant1,
    }

    let discriminants = SimpleDiscriminants::iter().collect::<Vec<_>>();
    let expected = vec![SimpleDiscriminants::Variant0, SimpleDiscriminants::Variant1];

    assert_eq!(expected, discriminants);
}

#[allow(dead_code)]
#[derive(EnumDiscriminants)]
#[repr(u16)]
enum WithReprUInt {
    Variant0,
    Variant1,
}

#[test]
fn with_repr_uint() {
    // These tests would not be proof of proper functioning on a 16 bit system
    assert_eq!(size_of::<u16>(), size_of::<WithReprUIntDiscriminants>());
    assert_eq!(
        size_of::<WithReprUInt>(),
        size_of::<WithReprUIntDiscriminants>()
    )
}

#[allow(dead_code)]
#[derive(EnumDiscriminants)]
#[repr(align(16), u8)]
enum WithReprAlign {
    Variant0,
    Variant1,
}

#[test]
fn with_repr_align() {
    assert_eq!(
        align_of::<WithReprAlign>(),
        align_of::<WithReprAlignDiscriminants>()
    );
    assert_eq!(16, align_of::<WithReprAlignDiscriminants>());
}

#[allow(dead_code)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(derive(FromRepr))]
enum WithExplicitDicriminantValue {
    Variant0 = 42 + 100,
    Variant1 = 11,
}

#[test]
fn with_explicit_discriminant_value() {
    assert_eq!(
        WithExplicitDicriminantValueDiscriminants::from_repr(11),
        Some(WithExplicitDicriminantValueDiscriminants::Variant1)
    );
    assert_eq!(
        142,
        WithExplicitDicriminantValueDiscriminants::Variant0 as u8
    );
}
