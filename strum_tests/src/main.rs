extern crate strum;
#[macro_use]
extern crate strum_macros;

#[allow(dead_code)]
#[derive(Debug,Eq,PartialEq,EnumString,ToString)]
enum Color {
    #[strum(to_string="RedRed", as_str="redred")]
    Red,
    #[strum(serialize="b", to_string="blue")]
    Blue { hue: usize },
    #[strum(serialize="y",serialize="yellow")]
    Yellow,
    #[strum(disabled="true")]
    Green(String),
}

fn main() {
    println!("Tests crate");
}
