use convert_case::{Case, Casing};
use std::{borrow::Cow, ops::Deref, str::FromStr};
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr,
};


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CaseStyle(Case<'static>);

impl Deref for CaseStyle {
    type Target = Case<'static>;
    fn deref(&self) -> &Self::Target {
        &self.0
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
        Ok(CaseStyle(match text {
            // Uniform case
            "snake" | "snek" |
            // Cased
            "snake_case" | "snek_case" => Case::Snake,

            // Uniform Case
            "upper_snek" | "upper_snake" | "screaming_snake" | "screaming_snek" |
            // Case
            "UPPER_SNEK_CASE" | "UPPER_SNAKE_CASE" | "SCREAMING_SNAKE_CASE" | "SCREAMING_SNEK_CASE" => {
                Case::UpperSnake
            }

            "ada" | "Ada_Case" => Case::Ada,
            "kebab" | "kebab-case" => Case::Kebab,

            // Uniform case
            "upper_kebab" | "screaming_kebab" |
            // Cased
            "UPPER-KEBAB-CASE" | "SCREAMING-KEBAB-CASE" => Case::UpperKebab,

            "train" | "Train-Case" => Case::Train,
            "flat" | "flatcase" => Case::Flat,
            
            // Uniform case
            "upper_flat" | "screaming_flat" |
            // Cased
            "UPPERFLATCASE" | "SCREAMINGFLATCASE" => Case::UpperFlat,

            "camel" | "camelCase" => Case::Camel,
            "pascal" | "PascalCase" => Case::Pascal,
            "lower" | "lower case" => Case::Lower,
            "upper" | "UPPER CASE" => Case::Upper,
            "title" | "Title Case" => Case::Title,
            "sentence" | "Sentence case" => Case::Sentence,
            "alternating" | "aLtErNaTiNg CaSe" => Case::Alternating,
            "toggle" | "tOGGLE cASE" => Case::Toggle,

            "lowercase" => return Err(CaseFromStrErr::InvalidLower),
            "UPPERCASE" => return Err(CaseFromStrErr::InvalidUpper),
            _invalid_str => return Err(CaseFromStrErr::UnknownVariant),
        }))
    }
}

pub trait CaseStyleHelpers {
    fn convert_case(&self, case_style: Option<CaseStyle>) -> String;
}

impl CaseStyleHelpers for Ident {
    fn convert_case(&self, case_style: Option<CaseStyle>) -> String {
        let ident_string = self.to_string();
        if let Some(case_style) = case_style {
            ident_string.to_case(*case_style)
        } else {
            ident_string
        }
    }
}

// compatibility with heck
pub fn snakify(s: &str) -> String {
    s.to_case(Case::Snake)
}
