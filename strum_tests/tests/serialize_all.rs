/// test `serialize_all` cooperation with other macroses
use std::str::FromStr;
use std::string::ToString;
use strum::{Display, EnumString, IntoStaticStr};

mod core {} // ensure macros call `::core`

#[derive(Debug, Eq, PartialEq, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "title_case")]
enum Foo1 {
    DarkBlack,
    Dim { glow: usize },
    BrightWhite,
}

#[test]
fn test_serialize_all_title_case() {
    assert_eq!("Dark Black", Foo1::DarkBlack.to_string());
    assert_eq!(Foo1::DarkBlack, Foo1::from_str("Dark Black").unwrap());
    assert_eq!("Dark Black", <&'static str>::from(Foo1::DarkBlack));
}

#[derive(Debug, Eq, PartialEq, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE")]
enum Foo2 {
    DarkBlack,
    Dim { glow: usize },
    BrightWhite,
}

#[test]
fn test_serialize_all_upper_case() {
    assert_eq!("DARKBLACK", Foo2::DarkBlack.to_string());
    assert_eq!(Foo2::DarkBlack, Foo2::from_str("DARKBLACK").unwrap());
    assert_eq!("DARKBLACK", <&'static str>::from(Foo2::DarkBlack));
}

// This is a soft-deprecated behavior. Use `camelCase` instead.
#[derive(Debug, Eq, PartialEq, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "camel_case")]
enum Foo3 {
    CamelCase,
}

#[test]
fn test_serialize_all_written_in_snake_case_camel_case() {
    assert_eq!("CamelCase", Foo3::CamelCase.to_string());
    assert_eq!(Foo3::CamelCase, Foo3::from_str("CamelCase").unwrap());
    assert_eq!("CamelCase", <&'static str>::from(Foo3::CamelCase));
}

#[derive(Debug, Eq, PartialEq, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "camelCase")]
enum Foo4 {
    CamelCase,
}

#[test]
fn test_serialize_all_camel_case() {
    assert_eq!("camelCase", Foo4::CamelCase.to_string());
    assert_eq!(Foo4::CamelCase, Foo4::from_str("camelCase").unwrap());
    assert_eq!("camelCase", <&'static str>::from(Foo4::CamelCase));
}
