#![allow(unused_imports)]
use strum::*;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, EnumString, ToString, EnumCount, EnumDiscriminants)]
pub enum Color {
    /// Docs on red
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(disabled)]
    Green(String),
}
