use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

mod core {} // ensure macros call `::core`

#[test]
fn simple() {
    #[allow(dead_code)]
    #[derive(EnumVariantNames)]
    enum Color {
        Red,
        #[strum(serialize = "b")]
        Blue,
        #[strum(to_string = "y", serialize = "yy")]
        Yellow,
    }

    assert_eq!(Color::VARIANTS, &["Red", "b", "y"]);
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
        assert_eq!(T::VARIANTS, &["Red", "Blue", "Yellow"]);
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

    assert_eq!(
        Color::VARIANTS,
        &["red", "blue", "yellow", "rebecca-purple"]
    );
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
}

#[test]
fn clap_and_structopt() {
    #[derive(Debug, StructOpt)]
    #[allow(unused)]
    struct StructOptExample {
        /// The main color
        #[structopt(
            long = "color",
            default_value = "Color::Blue",
            raw(possible_values = "Color::VARIANTS")
        )]
        color: Color,
    }

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

    let _clap_example = clap::App::new("app").arg(
        clap::Arg::with_name("color")
            .long("color")
            .possible_values(Color::VARIANTS)
            .case_insensitive(true),
    );
}

#[test]
fn crate_module_path_test() {
    pub mod nested {
        pub mod module {
            pub use strum;
        }
    }

    #[allow(dead_code)]
    #[derive(EnumVariantNames)]
    #[strum(crate = "nested::module::strum")]
    enum Color {
        Red,
        #[strum(serialize = "b")]
        Blue,
        #[strum(to_string = "y", serialize = "yy")]
        Yellow,
    }

    assert_eq!(Color::VARIANTS, &["Red", "b", "y"]);
}
