/// test `serialize_all` cooperation with other macroses
use std::str::FromStr;
use std::string::ToString;
use strum::{Display, EnumString, IntoStaticStr};

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
