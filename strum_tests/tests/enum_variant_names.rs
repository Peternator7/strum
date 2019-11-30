#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate structopt;
extern crate strum;
use strum::VariantNames;

#[test]
fn simple() {
    #[allow(dead_code)]
    #[derive(EnumVariantNames)]
    enum Color {
        Red,
        Blue,
        Yellow,
    }

    assert_eq!(Color::VARIANTS, &["Red", "Blue", "Yellow"]);
    assert_eq!(Color::variants(), &["Red", "Blue", "Yellow"]);
}

#[test]
fn variant_names_trait() {
    #[allow(dead_code)]
    #[derive(EnumVariantNames)]
    enum Color {
        Red,
        Blue,
        Yellow,
    }

    fn generic_function<T: VariantNames>() {
        assert_eq!(T::variants(), &["Red", "Blue", "Yellow"]);
    }

    generic_function::<Color>();
}

#[test]
fn plain_kebab() {
    #[allow(dead_code)]
    #[derive(EnumVariantNames)]
    #[strum(serialize_all = "kebab_case")]
    enum Color {
        Red,
        Blue,
        Yellow,
        RebeccaPurple,
    }

    assert_eq!(Color::VARIANTS, &["red", "blue", "yellow", "rebecca-purple"]);
    assert_eq!(Color::variants(), &["red", "blue", "yellow", "rebecca-purple"]);
}

#[test]
fn non_plain_camel() {
    #[allow(dead_code)]
    #[derive(EnumVariantNames)]
    #[strum(serialize_all = "kebab_case")]
    enum Color {
        DeepPink,
        GreenYellow,
        CornflowerBlue,
        Other { r: u8, g: u8, b: u8 },
    }

    assert_eq!(
        Color::VARIANTS,
        &["deep-pink", "green-yellow", "cornflower-blue", "other"]
    );
    assert_eq!(
        Color::variants(),
        &["deep-pink", "green-yellow", "cornflower-blue", "other"]
    );
}

#[test]
fn clap_and_structopt() {
    #[derive(Debug, EnumString, EnumVariantNames)]
    #[strum(serialize_all = "kebab_case")]
    enum Color {
        Red,
        Blue,
        Yellow,
        RebeccaPurple,
    }

    assert_eq!(
        Color::VARIANTS,
        &["red", "blue", "yellow", "rebecca-purple"]
    );

    assert_eq!(
        Color::variants(),
        &["red", "blue", "yellow", "rebecca-purple"]
    );

    let _clap_example = clap::App::new("app").arg(
        clap::Arg::with_name("color")
            .long("color")
            .possible_values(Color::VARIANTS)
            .case_insensitive(true),
    );

    #[derive(Debug, StructOpt)]
    #[allow(unused)]
    struct StructOptExample {
        /// The main color
        #[structopt(
            long = "color",
            default_value = "Color::Blue",
            raw(possible_values = "Color::variants()")
        )]
        color: Color,
    }
}
