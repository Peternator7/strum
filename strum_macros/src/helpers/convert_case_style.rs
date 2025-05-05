use convert_case::{Case, Casing};
use std::{borrow::Cow, str::FromStr};
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr,
};


#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CaseStyle {
    SnakeCase,
    UpperSnakeCase,
    AdaCase,
    KebabCase,
    UpperKebabCase,
    TrainCase,
    FlatCase,
    UpperFlatCase,
    CamelCase,
    PascalCase,
    LowerCase,
    UpperCase,
    TitleCase,
    SentenceCase,
    AlternatingCase,
    ToggleCase,
}

impl From<CaseStyle> for Case<'_> {
    fn from(value: CaseStyle) -> Self {
        match value {
            CaseStyle::SnakeCase => Case::Snake,
            CaseStyle::UpperSnakeCase => Case::UpperSnake,
            CaseStyle::AdaCase => Case::Ada,
            CaseStyle::KebabCase => Case::Kebab,
            CaseStyle::UpperKebabCase => Case::UpperKebab,
            CaseStyle::TrainCase => Case::Train,
            CaseStyle::FlatCase => Case::Flat,
            CaseStyle::UpperFlatCase => Case::UpperFlat,
            CaseStyle::CamelCase => Case::Camel,
            CaseStyle::LowerCase => Case::Lower,
            CaseStyle::UpperCase => Case::Upper,
            CaseStyle::TitleCase => Case::Title,
            CaseStyle::SentenceCase => Case::Sentence,
            CaseStyle::AlternatingCase => Case::Alternating,
            CaseStyle::ToggleCase => Case::Toggle,
            CaseStyle::PascalCase => Case::Pascal,
        }
    }
}

const VALID_CASE_STYLES: &[&str] = &[
    "snake_case",
    "UPPER_SNAKE_CASE",
    "Ada_Case",
    "kebab-case",
    "UPPER-KEBAB-CASE",
    "Train-Case",
    "flatcase",
    "UPPERFLATCASE",
    "camelCase",
    "PascalCase",
    "lower case",
    "UPPER CASE",
    "Title Case",
    "Sentence case",
    "aLtErNaTiNg CaSe",
    "tOGGLE cASE",
];

impl Parse for CaseStyle {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let syn_str = input.parse::<LitStr>()?;
        let string_value = syn_str.value();

        Self::from_str(syn_str.value().as_str()).map_err(|err| {
            syn::Error::new_spanned(
                &syn_str,
                match err {
                    CaseFromStrErr::InvalidLower => Cow::Borrowed(r#""lowercase" is not a valid case. Did you meant "lower case" or "flatcase"?"#),
                    CaseFromStrErr::InvalidUpper => Cow::Borrowed(r#""UPPERCASE" is not a valid case. Did you meant "UPPER CASE" or "UPPERFLATCASE"?"#),
                    // TODO: On <https://github.com/rust-lang/rust/issues/54140>:
                    // Make the note as `.span_note`
                    CaseFromStrErr::UnknownVariant => Cow::Owned(format!(
                        r#"Unexpected case style for serialize_all: `{}`.
Valid values are:
{}
Note: You can also write the name of the case in snake_case without "case" like so: `serialize = "upper_snake"`"#,
                        string_value, VALID_CASE_STYLES.iter().map(|s| format!("- {s}\n")).collect::<String>()
                    )),
                }
            )
        })
    }
}

pub enum CaseFromStrErr {
    InvalidLower,
    InvalidUpper,
    UnknownVariant,
}

impl FromStr for CaseStyle {
    type Err = CaseFromStrErr;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            // Uniform case
            "snake" | "snek" |
            // Cased
            "snake_case" | "snek_case" => CaseStyle::SnakeCase,

            // Uniform Case
            "upper_snek" | "upper_snake" | "screaming_snake" | "screaming_snek" |
            // Case
            "UPPER_SNEK_CASE" | "UPPER_SNAKE_CASE" | "SCREAMING_SNAKE_CASE" | "SCREAMING_SNEK_CASE" => {
                CaseStyle::UpperSnakeCase
            }

            "ada" | "Ada_Case" => Self::AdaCase,
            "kebab" | "kebab-case" => Self::KebabCase,

            // Uniform case
            "upper_kebab" | "screaming_kebab" |
            // Cased
            "UPPER-KEBAB-CASE" | "SCREAMING-KEBAB-CASE" => Self::UpperKebabCase,

            "train" | "Train-Case" => Self::TrainCase,
            "flat" | "flatcase" => Self::FlatCase,
            
            // Uniform case
            "upper_flat" | "screaming_flat" |
            // Cased
            "UPPERFLATCASE" | "SCREAMINGFLATCASE" => Self::UpperFlatCase,

            "camel" | "camelCase" => Self::CamelCase,
            "pascal" | "PascalCase" => Self::PascalCase,
            "lower" | "lower case" => Self::LowerCase,
            "upper" | "UPPER CASE" => Self::UpperCase,
            "title" | "Title Case" => Self::TitleCase,
            "sentence" | "Sentence case" => Self::SentenceCase,
            "alternating" | "aLtErNaTiNg CaSe" => Self::AlternatingCase,
            "toggle" | "tOGGLE cASE" => Self::ToggleCase,

            "lowercase" => return Err(CaseFromStrErr::InvalidLower),
            "UPPERCASE" => return Err(CaseFromStrErr::InvalidUpper),
            _invalid_str => return Err(CaseFromStrErr::UnknownVariant),
        })
    }
}

pub trait CaseStyleHelpers {
    fn convert_case(&self, case_style: Option<CaseStyle>) -> String;
}

impl CaseStyleHelpers for Ident {
    fn convert_case(&self, case_style: Option<CaseStyle>) -> String {
        let ident_string = self.to_string();
        if let Some(case_style) = case_style {
            ident_string.to_case(case_style.into())
        } else {
            ident_string
        }
    }
}

// compatibility with heck
pub fn snakify(s: &str) -> String {
    s.to_case(Case::Snake)
}
