//! Example howto use EnumVariantNames for structopt

// import the macros needed
use strum_macros::{EnumString, EnumVariantNames};
// You need to import the trait, to have access to VARIANTS
use structopt::StructOpt;
use strum::VariantNames;

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum Color {
    Red,
    Blue,
    Yellow,
    RebeccaPurple,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "EnumVariantNames",
    about = "example use of EnumVariantNames macro for structopt."
)]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Select color
    #[structopt(long, possible_values = &Color::VARIANTS)]
    color: Color,
}

fn main() {
    // verify variants
    assert_eq!(
        &Color::VARIANTS,
        &["red", "blue", "yellow", "rebecca-purple"]
    );
    // parse args and show them
    let opt = Opt::from_args();
    println!("options used debug: {} color: {:?}", opt.debug, opt.color);
}
