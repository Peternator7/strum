// Bring trait into scope
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumDiscriminants, EnumIter, EnumString};

#[derive(Debug)]
struct NonDefault;

// simple example
#[allow(dead_code)]
#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString))]
enum MyEnum {
    Variant0(NonDefault),
    Variant1 { a: NonDefault },
}

// You can also rename the generated enum using the `#[strum_discriminants(name(OtherName))]` attribute:
#[allow(dead_code)]
#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(name(MyVariants))]
enum MyEnumR {
    Variant0(bool),
    Variant1 { a: bool },
}

fn main() {
    // test simple example
    assert_eq!(
        MyEnumDiscriminants::Variant0,
        MyEnumDiscriminants::from_str("Variant0").unwrap()
    );
    // test rename example combined with EnumIter
    assert_eq!(
        vec![MyVariants::Variant0, MyVariants::Variant1],
        MyVariants::iter().collect::<Vec<_>>()
    );
}
