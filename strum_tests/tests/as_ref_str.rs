extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::str::FromStr;
use strum::AsStaticRef;

#[derive(Debug, Eq, PartialEq, EnumString, AsRefStr, AsStaticStr, IntoStaticStr)]
enum Color {
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(default = "true")]
    Green(String),
}

#[test]
fn as_red_str() {
    assert_eq!("RedRed", (Color::Red).as_ref());
    assert_eq!(Color::Red, Color::from_str((Color::Red).as_ref()).unwrap());
}

#[test]
fn as_blue_str() {
    assert_eq!("blue", (Color::Blue { hue: 0 }).as_ref());
    let _: &'static str = (Color::Blue { hue: 0 }).as_static();
}

#[test]
fn as_yellow_str() {
    assert_eq!("yellow", (Color::Yellow).as_ref());
    let _: &'static str = (Color::Yellow).as_static();
}

#[test]
fn as_green_str() {
    assert_eq!("Green", (Color::Green(String::default())).as_ref());
    let _: &'static str = (Color::Green(String::default())).as_static();
}

#[derive(IntoStaticStr)]
enum Foo<'a> {
    A,
    C(&'a i32),
}

#[derive(IntoStaticStr)]
enum Boo<'a, T> {
    A(T),
    B,
    C(&'a i32),
}

#[derive(IntoStaticStr)]
enum Moo<'a, T>
where
    T: AsRef<str>,
{
    A(T),
    B,
    C(&'a i32),
}

#[test]
fn test_into_static_str() {
    assert_eq!("RedRed", <&'static str>::from(Color::Red));
    assert_eq!("blue", <&'static str>::from(Color::Blue { hue: 0 }));
    assert_eq!("yellow", <&'static str>::from(Color::Yellow));

    assert_eq!("RedRed", <&'static str>::from(&Color::Red));
    assert_eq!("blue", <&'static str>::from(&Color::Blue { hue: 0 }));
    assert_eq!("yellow", <&'static str>::from(&Color::Yellow));

    assert_eq!("A", <&'static str>::from(Foo::A));
    assert_eq!("C", <&'static str>::from(Foo::C(&17)));

    assert_eq!("A", <&'static str>::from(Boo::A(17)));
    assert_eq!("B", <&'static str>::from(Boo::B::<i32>));
    assert_eq!("C", <&'static str>::from(Boo::C::<i32>(&17)));

    assert_eq!("A", <&'static str>::from(Moo::A::<String>("aaa".into())));
    assert_eq!("B", <&'static str>::from(Moo::B::<String>));
    assert_eq!("C", <&'static str>::from(Moo::C::<String>(&17)));
}

#[derive(Debug, Eq, PartialEq, AsRefStr, AsStaticStr, IntoStaticStr)]
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
    assert_eq!("dark_black", Brightness::DarkBlack.as_ref());
    assert_eq!("dim", Brightness::Dim { glow: 0 }.as_ref());
    assert_eq!("Bright", Brightness::BrightWhite.as_ref());

    assert_eq!("dark_black", Brightness::DarkBlack.as_static());
    assert_eq!("dim", Brightness::Dim { glow: 0 }.as_static());
    assert_eq!("Bright", Brightness::BrightWhite.as_static());

    assert_eq!("dark_black", <&'static str>::from(Brightness::DarkBlack));
    assert_eq!("dim", <&'static str>::from(Brightness::Dim { glow: 0 }));
    assert_eq!("Bright", <&'static str>::from(Brightness::BrightWhite));
}
