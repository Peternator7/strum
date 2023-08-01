use strum::{Display, EnumCount, EnumDiscriminants, EnumString};

#[derive(Debug, Eq, PartialEq, EnumString, Display, EnumCount, EnumDiscriminants)]
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
