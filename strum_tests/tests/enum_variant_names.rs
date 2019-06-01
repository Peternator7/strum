#[macro_use]
extern crate strum_macros;

#[test]
fn simple() {
    #[allow(dead_code)]
    #[derive(EnumVariantNames)]
    enum Color {
        Red,
        Blue,
        Yellow,
    }

    assert_eq!(&Color::variants(), &["Red", "Blue", "Yellow"]);
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
        &Color::variants(),
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
        &Color::variants(),
        &["deep-pink", "green-yellow", "cornflower-blue", "other"]
    );
}
