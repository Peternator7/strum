// You need to bring the trait into scope to use it!
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
enum Color {
    Red,
    Green { range: usize },
    Blue(usize),
    Yellow,
}

// It's simple to iterate over the variants of an enum.
fn debug_colors() {
    for color in Color::iter() {
        println!("My favorite color is {:?}", color);
    }
}

fn main() {
    debug_colors();
}
