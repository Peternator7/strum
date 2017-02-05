#[macro_use]
extern crate strum_macros;
extern crate strum;

use std::fmt::Debug;
use strum::IntoEnumIterator;

#[derive(Debug,Eq,PartialEq,EnumString,EnumIter,EnumHelp)]
enum Color {
    Red(usize),
    Blue { hue: usize },
    #[strum(serialize="y",serialize="yellow",help="This is the color yellow")]
    Yellow,
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
    debug_enum::<Color, _, _>(|color| println!("{:?}", color.get_detailed_help()));
}
