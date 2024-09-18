use std::str::FromStr;
use strum::EnumString;

mod core {} // ensure macros call `::core`

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
    Pink {
        #[strum(default_with = "test_default")]
        test_no_default: NoDefault,

        #[strum(default_with = "string_test")]
        string_test: String,
    },
    #[strum(default_with = "to_white")]
    White(String),
}

#[rustversion::since(1.34)]
fn assert_from_str<'a, T>(a: T, from: &'a str)
where
    T: PartialEq + std::str::FromStr + std::convert::TryFrom<&'a str> + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    <T as std::convert::TryFrom<&'a str>>::Error: std::fmt::Debug,
{
    assert_eq!(a, T::from_str(from).unwrap());
    assert_eq!(a, std::convert::TryFrom::try_from(from).unwrap());
}

#[rustversion::before(1.34)]
fn assert_from_str<T>(a: T, from: &str)
where
    T: PartialEq + std::str::FromStr + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    assert_eq!(a, T::from_str(from).unwrap());
}

#[test]
fn color_simple() {
    assert_from_str(Color::Red, "Red");
}

#[test]
fn color_value() {
    assert_from_str(Color::Blue { hue: 0 }, "Blue");
}

#[test]
fn color_serialize() {
    assert_from_str(Color::Yellow, "y");
    assert_from_str(Color::Yellow, "yellow");
}

#[test]
fn color_to_string() {
    assert_from_str(Color::Purple, "purp");
}

#[test]
fn color_default() {
    assert_from_str(Color::Green(String::from("not found")), "not found");
}

#[test]
fn color_ascii_case_insensitive() {
    assert_from_str(Color::Black, "BLK");
    assert_from_str(Color::Black, "bLaCk");
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
    assert_from_str(Brightness::DarkBlack, "dark_black");
    assert_from_str(Brightness::Dim { glow: 0 }, "dim");
    assert_from_str(Brightness::BrightWhite, "Bright");
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
    assert_from_str(Week::Sunday, "Sunday");
    assert_from_str(Week::Monday, "Monday");
    assert_from_str(Week::Tuesday, "Tuesday");
    assert_from_str(Week::Wednesday, "Wednesday");
    assert_from_str(Week::Thursday, "Thursday");
    assert_from_str(Week::Friday, "Friday");
    assert_from_str(Week::Saturday, "Saturday");
}

#[derive(Debug, Eq, PartialEq, EnumString)]
enum Lifetime<'a> {
    Life(&'a str),
    None,
}

#[test]
fn lifetime_test() {
    assert_from_str(Lifetime::Life(""), "Life");
}

#[derive(Debug, Eq, PartialEq, EnumString)]
enum Generic<T: Default> {
    Gen(T),
    Error,
}

#[test]
fn generic_test() {
    assert_from_str(Generic::Gen(""), "Gen");
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
    assert_from_str(CaseInsensitiveEnum::NoAttr, "noattr");
}

#[test]
fn case_insensitive_enum_no_case_insensitive() {
    assert_from_str(CaseInsensitiveEnum::NoCaseInsensitive, "NoCaseInsensitive");
    assert!(CaseInsensitiveEnum::from_str("nocaseinsensitive").is_err());
}

#[rustversion::since(1.34)]
#[test]
fn case_insensitive_enum_no_case_insensitive_try_from() {
    assert_from_str(CaseInsensitiveEnum::NoCaseInsensitive, "NoCaseInsensitive");
    assert!(
        <CaseInsensitiveEnum as std::convert::TryFrom<&str>>::try_from("nocaseinsensitive")
            .is_err()
    );
}

#[test]
fn case_insensitive_enum_case_insensitive() {
    assert_from_str(CaseInsensitiveEnum::CaseInsensitive, "CaseInsensitive");
    assert_from_str(CaseInsensitiveEnum::CaseInsensitive, "caseinsensitive");
}

#[derive(Eq, PartialEq, Debug)]
struct NoDefault(String);

fn test_default() -> NoDefault {
    NoDefault(String::from("test"))
}

fn to_white() -> String {
    String::from("white-test")
}

fn string_test() -> String {
    String::from("This is a string test")
}

#[test]
fn color_default_with() {
    match Color::from_str("Pink").unwrap() {
        Color::Pink {
            test_no_default,
            string_test,
        } => {
            assert_eq!(test_no_default, test_default());
            assert_eq!(string_test, String::from("This is a string test"));
        }
        other => {
            panic!("Failed to get correct enum value {:?}", other);
        }
    }
}

#[test]
fn color_default_with_white() {
    match Color::from_str("White").unwrap() {
        Color::White(inner) => {
            assert_eq!(inner, String::from("white-test"));
        }
        other => {
            panic!("Failed t o get  correct enum value {:?}", other);
        }
    }
}

#[derive(Debug, EnumString)]
#[strum(
    parse_err_fn = "some_enum_not_found_err",
    parse_err_ty = "CaseCustomParseErrorNotFoundError"
)]
enum CaseCustomParseErrorEnum {
    #[strum(serialize = "red")]
    Red,
    #[strum(serialize = "blue")]
    Blue,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct CaseCustomParseErrorNotFoundError(String);
impl std::fmt::Display for CaseCustomParseErrorNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "not found `{}`", self.0)
    }
}
impl std::error::Error for CaseCustomParseErrorNotFoundError {}
fn some_enum_not_found_err(s: &str) -> CaseCustomParseErrorNotFoundError {
    CaseCustomParseErrorNotFoundError(s.to_string())
}

#[test]
fn case_custom_parse_error() {
    let r = "yellow".parse::<CaseCustomParseErrorEnum>();
    assert!(r.is_err());
    assert_eq!(
        CaseCustomParseErrorNotFoundError("yellow".to_string()),
        r.unwrap_err()
    );
}
