//! Encode strings into the enum itself. The `strum_macros::EmumMessage` macro implements the `strum::EnumMessage` trait.
//! `EnumMessage` looks for `#[strum(message="...")]` attributes on your variants.
//! You can also provided a `detailed_message="..."` attribute to create a seperate more detailed message than the first.

// You need to bring the trait into scope to use it!!!
use strum::EnumMessage;
use strum_macros::EnumMessage;

#[derive(EnumMessage, Debug)]
#[allow(dead_code)]
enum Color {
    #[strum(message = "Red", detailed_message = "This is very red")]
    Red,
    #[strum(message = "Simply Green")]
    Green { range: usize },
    #[strum(serialize = "b", serialize = "blue")]
    Blue(usize),
}

// Generated code looks like more or less like this:
/*
impl ::strum::EnumMessage for Color {
    fn get_message(&self) -> ::std::option::Option<&str> {
        match self {
            &Color::Red => ::std::option::Option::Some("Red"),
            &Color::Green {..} => ::std::option::Option::Some("Simply Green"),
            _ => None
        }
    }

    fn get_detailed_message(&self) -> ::std::option::Option<&str> {
        match self {
            &Color::Red => ::std::option::Option::Some("This is very red"),
            &Color::Green {..}=> ::std::option::Option::Some("Simply Green"),
            _ => None
        }
    }

    fn get_serializations(&self) -> &[&str] {
        match self {
            &Color::Red => {
                static ARR: [&'static str; 1] = ["Red"];
                &ARR
            },
            &Color::Green {..}=> {
                static ARR: [&'static str; 1] = ["Green"];
                &ARR
            },
            &Color::Blue (..) => {
                static ARR: [&'static str; 2] = ["b", "blue"];
                &ARR
            },
        }
    }
}
*/
fn main() {
    let c = Color::Red;
    println!("{}", c.get_message().unwrap());
    println!("{}", c.get_detailed_message().unwrap());
    println!("{:?}", c.get_serializations());
}
