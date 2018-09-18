#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaseStyle {
    CamelCase,
    KebabCase,
    MixedCase,
    ShoutySnakeCase,
    SnakeCase,
    TitleCase,
}

impl<'s> From<&'s str> for CaseStyle {
    fn from(text: &'s str) -> CaseStyle {
        match text {
            "camel_case" => CaseStyle::CamelCase,
            "kebab_case" => CaseStyle::KebabCase,
            "mixed_case" => CaseStyle::MixedCase,
            "shouty_snake_case" | "shouty_snek_case" => CaseStyle::ShoutySnakeCase,
            "snake_case" | "snek_case" => CaseStyle::SnakeCase,
            "title_case" => CaseStyle::TitleCase,
            _ => panic!(
                "Unexpected case style for serialize_all: `{}`. Valid values are: `{:?}`",
                text,
                [
                    "camel_case",
                    "kebab_case",
                    "mixed_case",
                    "shouty_snake_case",
                    "snake_case",
                    "title_case"
                ]
            ),
        }
    }
}
