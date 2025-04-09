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

#[cfg(feature = "test_phf")]
#[test]
fn infallible_from_str_with_phf() {
    // This tests PHF with infallible parsing
    #[derive(Debug, PartialEq, Eq, Clone, strum::EnumString)]
    #[strum(use_phf, parse_infallible)]
    enum InfallibleColor {
        #[strum(ascii_case_insensitive)]
        Blue,
        Red,
        #[strum(default)]
        Unknown(String),
    }
    assert_eq!(InfallibleColor::from("Red"), InfallibleColor::Red);
    assert_eq!(InfallibleColor::from("bLuE"), InfallibleColor::Blue);
    assert_eq!("Red".parse(), Ok(InfallibleColor::Red));
    assert_eq!("bLuE".parse(), Ok(InfallibleColor::Blue));
}
