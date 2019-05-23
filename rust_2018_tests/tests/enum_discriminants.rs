#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum_macros::EnumDiscriminants)]
#[strum_discriminants(name(SplitAttributesBoo), derive(strum_macros::Display))]
#[strum_discriminants(derive(strum_macros::EnumIter))]
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
#[derive(Debug, Eq, PartialEq, strum_macros::EnumDiscriminants)]
#[strum_discriminants(
    name(PassThroughBoo),
    derive(strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString),
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
