#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaseStyle {
    CamelCase,
    KebabCase,
    MixedCase,
    ShoutySnakeCase,
    SnakeCase,
    TitleCase,
    UpperCase,
    LowerCase,
    ScreamingKebabCase,
    PascalCase,
}

impl<'s> From<&'s str> for CaseStyle {
    fn from(text: &'s str) -> CaseStyle {
        match text {
            "lowercase" => CaseStyle::LowerCase,
            "camel_case" | "PascalCase" => CaseStyle::PascalCase,
            "kebab_case" | "kebab-case" => CaseStyle::KebabCase,
            "mixed_case" => CaseStyle::MixedCase,
            "shouty_snake_case" | "shouty_snek_case" | "SCREAMING_SNAKE_CASE" => {
                CaseStyle::ShoutySnakeCase
            }
            "snake_case" | "snek_case" => CaseStyle::SnakeCase,
            "title_case" => CaseStyle::TitleCase,
            "UPPERCASE" => CaseStyle::UpperCase,
            "camelCase" => CaseStyle::CamelCase,
            "SCREAMING-KEBAB-CASE" => CaseStyle::ScreamingKebabCase,
            _ => panic!(
                "Unexpected case style for serialize_all: `{}`. Valid values are: `{:?}`",
                text,
                [
                    "camel_case",
                    "kebab_case",
                    "mixed_case",
                    "shouty_snake_case",
                    "snake_case",
                    "title_case",
                    "UPPERCASE",
                ]
            ),
        }
    }
}
