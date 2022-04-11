#[rustversion::since(1.46)]
#[test]
fn from_str_with_phf() {
    #[derive(Debug, PartialEq, Eq, Clone, strum::EnumString)]
    #[strum(use_phf)]
    enum Color {
        Blue,
        Red,
    }
    assert_eq!("Blue".parse::<Color>().unwrap(), Color::Blue);
}

#[rustversion::since(1.46)]
#[test]
fn from_str_with_phf_case_insensitive() {
    #[derive(Debug, PartialEq, Eq, Clone, strum::EnumString)]
    #[strum(use_phf, ascii_case_insensitive)]
    enum Color {
        Blue,
        Red,
    }
    assert_eq!("bLuE".parse::<Color>().unwrap(), Color::Blue);
}
