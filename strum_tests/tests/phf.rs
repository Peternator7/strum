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

#[cfg(feature = "test_phf")]
#[test]
fn from_str_with_phf_lowercase() {
    #[derive(Debug, PartialEq, Eq, Clone, strum::EnumString)]
    #[strum(serialize_all = "lowercase", ascii_case_insensitive, use_phf)]
    enum Color {
        Red,
        Blue,
    }

    for input in ["red", "RED", "rEd"] {
        assert_eq!(input.parse::<Color>().unwrap(), Color::Red);
    }
    assert_eq!("BLUE".parse::<Color>().unwrap(), Color::Blue);
    assert!("unknown".parse::<Color>().is_err());
}

#[cfg(feature = "test_phf")]
#[test]
fn from_str_with_phf_uppercase() {
    #[derive(Debug, PartialEq, Eq, Clone, strum::EnumString)]
    #[strum(serialize_all = "UPPERCASE", ascii_case_insensitive, use_phf)]
    enum Color {
        Red,
    }

    for input in ["red", "RED", "rEd"] {
        assert_eq!(input.parse::<Color>().unwrap(), Color::Red);
    }
}

#[cfg(feature = "test_phf")]
#[test]
fn from_str_with_phf_case_insensitive_aliases() {
    #[derive(Debug, PartialEq, Eq, Clone, strum::EnumString)]
    #[strum(ascii_case_insensitive, use_phf)]
    enum Value {
        #[strum(serialize = "one", serialize = "ONE", serialize = "first")]
        One,
        #[strum(serialize = "123")]
        Digits,
        #[strum(serialize = "")]
        Empty,
        #[strum(serialize = "é")]
        NonAscii,
        #[strum(ascii_case_insensitive = false)]
        Exact,
    }

    for input in ["one", "ONE", "oNe", "first", "FIRST", "fIrSt"] {
        assert_eq!(input.parse::<Value>().unwrap(), Value::One);
    }
    assert_eq!("123".parse::<Value>().unwrap(), Value::Digits);
    assert_eq!("".parse::<Value>().unwrap(), Value::Empty);
    assert_eq!("é".parse::<Value>().unwrap(), Value::NonAscii);
    assert!("É".parse::<Value>().is_err());
    assert_eq!("Exact".parse::<Value>().unwrap(), Value::Exact);
    assert!("exact".parse::<Value>().is_err());
}

#[cfg(feature = "test_phf")]
#[test]
fn from_str_with_phf_lowercase_default() {
    #[derive(Debug, PartialEq, Eq, Clone, strum::EnumString)]
    #[strum(serialize_all = "lowercase", ascii_case_insensitive, use_phf)]
    enum Color {
        Red,
        #[strum(default)]
        Unknown(String),
    }

    for input in ["red", "RED", "rEd"] {
        assert_eq!(Color::from(input), Color::Red);
    }
    assert_eq!(
        Color::from("unknown"),
        Color::Unknown("unknown".to_string())
    );
}
