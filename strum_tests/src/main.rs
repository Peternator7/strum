extern crate strum;
#[macro_use]
extern crate strum_macros;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, EnumString, ToString, EnumCount, EnumDiscriminants)]
enum Color {
    /// Random Docs
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(disabled = "true")]
    Green(String),
}

fn main() {
    println!("Tests crate");
}
