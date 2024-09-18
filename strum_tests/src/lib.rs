use strum::{Display, EnumCount, EnumDiscriminants, EnumString};
use strum_macros::EnumIs;

#[derive(Debug, Eq, PartialEq, EnumString, Display, EnumCount, EnumDiscriminants, EnumIs)]
pub enum Color {
    /// Docs on red
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(disabled)]
    Green(String),
}

#[cfg(test)]
mod tests {
    use strum::EnumString;

    #[derive(Debug, EnumString)]
    #[strum(
        parse_err_fn = "self::some_enum_not_found_err",
        parse_err_ty = "self::NotFoundError"
    )]
    enum Color {
        /// Docs on red
        #[strum(serialize = "red")]
        Red,
        #[strum(serialize = "blue")]
        Blue,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct NotFoundError(String);
    impl std::fmt::Display for NotFoundError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "not found `{}`", self.0)
        }
    }
    impl std::error::Error for NotFoundError {}

    fn some_enum_not_found_err(s: &str) -> NotFoundError {
        NotFoundError(s.to_string())
    }

    #[test]
    fn test_custom_err() {
        let r = "yellow".parse::<Color>();
        assert!(r.is_err());
        assert_eq!(NotFoundError("yellow".to_string()), r.unwrap_err());
    }
}
