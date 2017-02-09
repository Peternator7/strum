extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::{IntoEnumIterator, EnumMessage};

#[derive(Debug,Eq,PartialEq,EnumString,EnumIter,EnumMessage)]
enum Color<T: Default> {
    #[strum(message="The color red")]
    Red(usize),
    #[strum(message="Blue blue")]
    Blue { hue: usize },
    #[strum(serialize="y",serialize="yellow",message="This is the color yellow", detailed_message="This is the detailed message.")]
    Yellow,
    #[strum(default="true",message="greenies")]
    Green(String),
    Pink(T),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
