mod core {} // ensure macros call `::core`

#[cfg(feature = "test_phf")]
#[test]
fn from_str_with_phf() {
    #[derive(Debug, PartialEq, Eq, Clone, strum::EnumString)]
    #[strum(use_phf)]
    enum Color {
        #[strum(ascii_case_insensitive)]
        Blue,
        Red,
    }
    assert_eq!("Red".parse::<Color>().unwrap(), Color::Red);
    assert_eq!("bLuE".parse::<Color>().unwrap(), Color::Blue);
}

#[cfg(feature = "test_phf")]
#[test]
fn from_str_with_phf_infallible() {
    #[derive(Debug, PartialEq, Eq, Clone, strum::EnumString)]
    #[strum(use_phf)]
    enum Color {
        Red,
        Blue,
        #[strum(default)]
        Unknown(String),
    }

    // Known variants still parse correctly
    let c: Color = Color::from("Red");
    assert_eq!(c, Color::Red);
    let c: Color = Color::from("Blue");
    assert_eq!(c, Color::Blue);

    // Unknown input falls through to the default variant
    let c: Color = Color::from("notacolor");
    assert_eq!(c, Color::Unknown("notacolor".to_string()));
}

#[cfg(feature = "test_phf")]
#[test]
fn from_str_with_phf_infallible_case_insensitive() {
    #[derive(Debug, PartialEq, Eq, Clone, strum::EnumString)]
    #[strum(use_phf)]
    enum Color {
        #[strum(ascii_case_insensitive)]
        Blue,
        Red,
        #[strum(default)]
        Unknown(String),
    }

    let c: Color = Color::from("bLuE");
    assert_eq!(c, Color::Blue);
    let c: Color = Color::from("notacolor");
    assert_eq!(c, Color::Unknown("notacolor".to_string()));
}

#[cfg(feature = "test_phf")]
#[test]
fn from_str_with_phf_big() {
    // This tests PHF when there are many case insensitive variants
    #[derive(Debug, PartialEq, Eq, Clone, strum::EnumString)]
    #[strum(use_phf, ascii_case_insensitive)]
    enum Enum {
        Var1,
        Var2,
        Var3,
        Var4,
        Var5,
        Var6,
        Var7,
        Var8,
        Var9,
    }
    assert_eq!("vAr2".parse::<Enum>().unwrap(), Enum::Var2);
}
