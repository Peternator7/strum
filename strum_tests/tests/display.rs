use strum::EnumString;

mod core {} // ensure macros call `::core`

#[derive(Debug, Eq, PartialEq, EnumString, strum::Display)]
enum InnerColor {
    Violet,
    Fuchsia,
}

impl Default for InnerColor {
    fn default() -> Self {
        InnerColor::Violet
    }
}

#[derive(Debug, Eq, PartialEq, EnumString, strum::Display)]
enum Color {
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(to_string = "saturation is {sat}")]
    Purple { sat: usize },
    #[strum(default)]
    Green(String),
    #[strum(to_string = "Orange({0})")]
    Orange(usize),
    #[strum(transparent)]
    Inner(InnerColor),
}

#[test]
fn to_blue_string() {
    assert_eq!(String::from("blue"), format!("{}", Color::Blue { hue: 0 }));
}

#[test]
fn test_formatters() {
    assert_eq!(
        String::from("  blue"),
        format!("{:>6}", Color::Blue { hue: 0 })
    );
    assert_eq!(
        String::from("blue  "),
        format!("{:<6}", Color::Blue { hue: 0 })
    );
    assert_eq!(
        String::from(" blue "),
        format!("{:^6}", Color::Blue { hue: 0 })
    );
    assert_eq!(String::from("bl"), format!("{:.2}", Color::Blue { hue: 0 }));
}

#[test]
fn to_yellow_string() {
    assert_eq!(String::from("yellow"), format!("{}", Color::Yellow));
}

#[test]
fn to_purple_string() {
    assert_eq!(
        String::from("saturation is 10"),
        (Color::Purple { sat: 10 }).to_string().as_ref()
    );
}

#[test]
fn to_red_string() {
    assert_eq!(String::from("RedRed"), format!("{}", Color::Red));
}

#[test]
fn to_green_string() {
    assert_eq!(
        String::from("  lime"),
        format!("{:>6}", Color::Green("lime".into()))
    );
}

#[test]
fn to_orange_string() {
    assert_eq!(
        String::from("Orange(10)"),
        Color::Orange(10).to_string().as_ref()
    )
}

#[test]
fn to_violet_string() {
    assert_eq!(
        String::from("Violet"),
        format!("{}", Color::Inner(InnerColor::Violet))
    );
}

#[derive(Debug, Eq, PartialEq, EnumString, strum::Display)]
enum ColorWithDefaultAndToString {
    #[strum(default, to_string = "GreenGreen")]
    Green(String),
}

#[test]
fn to_green_with_default_and_to_string() {
    assert_eq!(
        String::from("GreenGreen"),
        format!("{}", ColorWithDefaultAndToString::Green("lime".into()))
    );
}

#[derive(strum::Display, Debug, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum Brightness {
    DarkBlack,
    Dim {
        glow: usize,
    },
    #[strum(serialize = "bright")]
    BrightWhite,
}

#[test]
fn brightness_to_string() {
    assert_eq!(
        String::from("dark_black"),
        Brightness::DarkBlack.to_string().as_ref()
    );
    assert_eq!(
        String::from("dim"),
        Brightness::Dim { glow: 0 }.to_string().as_ref()
    );
    assert_eq!(
        String::from("bright"),
        Brightness::BrightWhite.to_string().as_ref()
    );
}

#[derive(strum::Display, Debug, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum NonStringDefault {
    #[strum(default)]
    Number(usize),
}

#[test]
fn non_string_default_to_string() {
    assert_eq!(
        String::from("0014"),
        format!("{:04}", NonStringDefault::Number(14))
    );
}

#[derive(strum::Display, Debug, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum TransparentString {
    #[strum(transparent)]
    Something(String),

    #[strum(transparent)]
    SomethingNamed { my_field: String },
}

#[test]
fn transparent_string() {
    assert_eq!(
        String::from("string in here"),
        format!(
            "{}",
            TransparentString::Something("string in here".to_owned())
        )
    );
}

#[test]
fn transparent_string_named_field() {
    assert_eq!(
        String::from("string in here"),
        format!(
            "{}",
            TransparentString::SomethingNamed {
                my_field: String::from("string in here")
            }
        )
    );
}
