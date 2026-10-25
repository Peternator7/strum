use std::str::FromStr;
use strum::{EnumString, EnumStringConst, ParseError};

mod core {} // ensure macros call `::core`

#[derive(Debug, Eq, PartialEq, EnumStringConst)]
enum Color {
    Red,
    #[strum(serialize = "b", serialize = "blue")]
    Blue,
    #[strum(to_string = "purp")]
    Purple,
    #[strum(serialize = "blk", serialize = "Black", ascii_case_insensitive)]
    Black,
    #[allow(dead_code)]
    #[strum(disabled)]
    Green,
}

#[test]
fn color_simple() {
    const RED: Result<Color, ParseError> = Color::from_str_const("Red");
    assert_eq!(Ok(Color::Red), RED);
}

#[test]
fn color_serialize() {
    const BLUE: Result<Color, ParseError> = Color::from_str_const("blue");
    assert_eq!(Ok(Color::Blue), BLUE);
    assert_eq!(Ok(Color::Blue), Color::from_str_const("b"));
}

#[test]
fn color_to_string() {
    assert_eq!(Ok(Color::Purple), Color::from_str_const("purp"));
}

#[test]
fn color_ascii_case_insensitive() {
    const BLACK: Result<Color, ParseError> = Color::from_str_const("bLaCk");
    assert_eq!(Ok(Color::Black), BLACK);
    assert_eq!(Ok(Color::Black), Color::from_str_const("BLK"));

    // Only variants marked ascii_case_insensitive should match loosely.
    assert_eq!(
        Err(ParseError::VariantNotFound),
        Color::from_str_const("red")
    );
}

#[test]
fn color_not_found() {
    const MISSING: Result<Color, ParseError> = Color::from_str_const("Orange");
    assert_eq!(Err(ParseError::VariantNotFound), MISSING);
    assert_eq!(
        Err(ParseError::VariantNotFound),
        Color::from_str_const("Green")
    );
}

#[derive(Debug, Eq, PartialEq, EnumString, EnumStringConst)]
#[strum(serialize_all = "snake_case")]
enum Brightness {
    DarkBlack,
    #[strum(serialize = "Bright")]
    BrightWhite,
}

#[test]
fn brightness_serialize_all() {
    const DARK_BLACK: Result<Brightness, ParseError> = Brightness::from_str_const("dark_black");
    assert_eq!(Ok(Brightness::DarkBlack), DARK_BLACK);
    assert_eq!(
        Ok(Brightness::BrightWhite),
        Brightness::from_str_const("Bright")
    );
}

#[test]
fn brightness_agrees_with_enum_string() {
    // Both derives should parse the same strings to the same variants.
    for s in ["dark_black", "Bright", "DarkBlack", "nope"] {
        assert_eq!(
            Brightness::from_str(s).ok(),
            Brightness::from_str_const(s).ok()
        );
    }
}

#[derive(Debug, Eq, PartialEq, EnumStringConst)]
#[strum(
    parse_err_fn = const_not_found_err,
    parse_err_ty = ConstNotFoundError
)]
enum ConstCustomParseErrorEnum {
    Red,
    Blue,
}

#[derive(Debug, Eq, PartialEq)]
struct ConstNotFoundError;

// The given parse_err_fn has to be a const fn for from_str_const to work.
const fn const_not_found_err(_s: &str) -> ConstNotFoundError {
    ConstNotFoundError
}

#[test]
fn custom_parse_error() {
    const RED: Result<ConstCustomParseErrorEnum, ConstNotFoundError> =
        ConstCustomParseErrorEnum::from_str_const("Red");
    assert_eq!(Ok(ConstCustomParseErrorEnum::Red), RED);

    const MISSING: Result<ConstCustomParseErrorEnum, ConstNotFoundError> =
        ConstCustomParseErrorEnum::from_str_const("yellow");
    assert_eq!(Err(ConstNotFoundError), MISSING);
}
