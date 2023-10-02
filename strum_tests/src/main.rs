use strum::{Display, EnumCount, EnumDiscriminants, EnumString};
use strum_macros::EnumIs;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, EnumString, Display, EnumCount, EnumDiscriminants, EnumIs)]
enum Color {
    /// Random Docs
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(disabled)]
    Green(String),
}

fn main() {
    println!("Tests crate");
}
