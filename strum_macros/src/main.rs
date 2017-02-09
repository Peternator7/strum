#[macro_use]
extern crate strum_macros;
extern crate strum;

use std::fmt::Debug;
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


fn debug_enum<E, F, I: Iterator<Item = E>>(pred: F)
    where E: IntoEnumIterator<Iterator = I>,
          F: Fn(E)
{
    for e in E::iter() {
        pred(e);
    }
}

pub fn main() {
    debug_enum::<Color<usize>, _, _>(|color| println!("{:?}", color.get_detailed_message()));
}
