use strum_macros::EnumProperty;
// bring the trait into scope
use strum::EnumProperty;

#[derive(EnumProperty, Debug)]
#[allow(dead_code)]
enum Color {
    #[strum(props(Red = "255", Blue = "255", Green = "255"))]
    White,
    #[strum(props(Red = "0", Blue = "0", Green = "0"))]
    Black,
    #[strum(props(Red = "0", Blue = "255", Green = "0"))]
    Blue,
    #[strum(props(Red = "255", Blue = "0", Green = "0"))]
    Red,
    #[strum(props(Red = "0", Blue = "0", Green = "255"))]
    Green,
}

fn main() {
    let my_color = Color::Red;
    let display = format!(
        "My color is {:?}. It's RGB is {},{},{}",
        my_color,
        my_color.get_str("Red").unwrap(),
        my_color.get_str("Green").unwrap(),
        my_color.get_str("Blue").unwrap()
    );
    println!("{}", display);
}
