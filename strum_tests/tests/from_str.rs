use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, Eq, PartialEq, EnumString)]
enum Color {
    Red,
    Blue {
        hue: usize,
    },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(default)]
    Green(String),
    #[strum(to_string = "purp")]
    Purple,
    #[strum(serialize = "blk", serialize = "Black", ascii_case_insensitive)]
    Black,
}

#[test]
fn color_simple() {
    assert_eq!(Color::Red, Color::from_str("Red").unwrap());
}

#[test]
fn color_value() {
    assert_eq!(Color::Blue { hue: 0 }, Color::from_str("Blue").unwrap());
}

#[test]
fn color_serialize() {
    assert_eq!(Color::Yellow, Color::from_str("y").unwrap());
    assert_eq!(Color::Yellow, Color::from_str("yellow").unwrap());
}

#[test]
fn color_to_string() {
    assert_eq!(Color::Purple, Color::from_str("purp").unwrap());
}

#[test]
fn color_default() {
    assert_eq!(
        Color::Green(String::from("not found")),
        Color::from_str("not found").unwrap()
    );
}

#[test]
fn color_ascii_case_insensitive() {
    assert_eq!(Color::Black, Color::from_str("BLK").unwrap());
    assert_eq!(Color::Black, Color::from_str("bLaCk").unwrap());
}

#[derive(Debug, Eq, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
enum Brightness {
    DarkBlack,
    Dim {
        glow: usize,
    },
    #[strum(serialize = "Bright")]
    BrightWhite,
}

#[test]
fn brightness_serialize_all() {
    assert_eq!(
        Brightness::DarkBlack,
        Brightness::from_str("dark_black").unwrap()
    );
    assert_eq!(
        Brightness::Dim { glow: 0 },
        Brightness::from_str("dim").unwrap()
    );
    assert_eq!(
        Brightness::BrightWhite,
        Brightness::from_str("Bright").unwrap()
    );
}

#[derive(Debug, Eq, PartialEq, EnumString)]
enum Week {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[test]
fn week_not_found() {
    assert_eq!(
        Result::Err(::strum::ParseError::VariantNotFound),
        Week::from_str("Humpday")
    );
}

#[test]
fn week_found() {
    assert_eq!(Result::Ok(Week::Sunday), Week::from_str("Sunday"));
    assert_eq!(Result::Ok(Week::Monday), Week::from_str("Monday"));
    assert_eq!(Result::Ok(Week::Tuesday), Week::from_str("Tuesday"));
    assert_eq!(Result::Ok(Week::Wednesday), Week::from_str("Wednesday"));
    assert_eq!(Result::Ok(Week::Thursday), Week::from_str("Thursday"));
    assert_eq!(Result::Ok(Week::Friday), Week::from_str("Friday"));
    assert_eq!(Result::Ok(Week::Saturday), Week::from_str("Saturday"));
}

#[derive(Debug, Eq, PartialEq, EnumString)]
enum Lifetime<'a> {
    Life(&'a str),
    None,
}

#[test]
fn lifetime_test() {
    assert_eq!(Lifetime::Life(""), Lifetime::from_str("Life").unwrap());
}

#[derive(Debug, Eq, PartialEq, EnumString)]
enum Generic<T: Default> {
    Gen(T),
    None,
}

#[test]
fn generic_test() {
    assert_eq!(Generic::Gen(""), Generic::from_str("Gen").unwrap());
}

#[derive(Debug, Eq, PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
enum CaseInsensitiveEnum {
    NoAttr,
    #[strum(ascii_case_insensitive = false)]
    NoCaseInsensitive,
    #[strum(ascii_case_insensitive = true)]
    CaseInsensitive,
}

#[test]
fn case_insensitive_enum_no_attr() {
    assert_eq!(
        CaseInsensitiveEnum::NoAttr,
        CaseInsensitiveEnum::from_str("noattr").unwrap()
    );
}

#[test]
fn case_insensitive_enum_no_case_insensitive() {
    assert_eq!(
        CaseInsensitiveEnum::NoCaseInsensitive,
        CaseInsensitiveEnum::from_str("NoCaseInsensitive").unwrap(),
    );
    assert!(CaseInsensitiveEnum::from_str("nocaseinsensitive").is_err());
}

#[test]
fn case_insensitive_enum_case_insensitive() {
    assert_eq!(
        CaseInsensitiveEnum::CaseInsensitive,
        CaseInsensitiveEnum::from_str("CaseInsensitive").unwrap(),
    );
    assert_eq!(
        CaseInsensitiveEnum::CaseInsensitive,
        CaseInsensitiveEnum::from_str("caseinsensitive").unwrap(),
    );
}
