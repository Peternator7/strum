#[macro_use]
extern crate macro_test;
extern crate strum;

use strum::IntoEnumIterator;

#[derive(Debug,Eq,PartialEq,FromString,EnumIter,EnumHelp)]
enum Color {
    Red(usize),
    Blue { hue: usize },
    #[strum(serialize="y",serialize="yellow",help="This is the color yellow")]
    Yellow,
}

#[derive(EnumHelp)]
enum Gender {
    #[strum(serialize="-b",serialize="-boy", help="I'm a boy")]
    Boy(usize),
    Girl(char),
}

pub fn main() {
    println!("Hello world");

    for color in Color::iter() {
        if let Some(msg) = color.get_help() {
            println!("{}", msg);
        }

        if let Some(msg) = color.get_detailed_help() {
            println!("{}", msg);
        }
    }

    println!("{}", (Gender::Boy(1)).get_help().unwrap());
}