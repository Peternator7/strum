extern crate strum;
#[macro_use]
extern crate strum_macros;

#[derive(StrumEnumString,
    StrumAsRefStr,
    StrumAsStaticStr,
    StrumIntoStaticStr,
    StrumDisplay,
    StrumEnumIter,
    StrumEnumMessage,
    StrumEnumProperty,
    StrumEnumDiscriminants,
    StrumEnumCount)]
pub enum Color {
    Red,
    Blue,
    Green,
}

// You can't have ToString and Display on the same type.
#[derive(StrumToString)]
pub enum Color2 {
    Red,
    Blue,
    Green
}