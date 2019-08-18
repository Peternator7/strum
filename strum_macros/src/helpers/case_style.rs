use heck::{CamelCase, KebabCase, MixedCase, ShoutySnakeCase, SnakeCase, TitleCase};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
            "camel_case" | "PascalCase" => CaseStyle::PascalCase,
            "camelCase" => CaseStyle::CamelCase,
            "snake_case" | "snek_case" => CaseStyle::SnakeCase,
            "kebab_case" | "kebab-case" => CaseStyle::KebabCase,
            "SCREAMING-KEBAB-CASE" => CaseStyle::ScreamingKebabCase,
            "shouty_snake_case" | "shouty_snek_case" | "SCREAMING_SNAKE_CASE" => {
                CaseStyle::ShoutySnakeCase
            }
            "title_case" => CaseStyle::TitleCase,
            "mixed_case" => CaseStyle::MixedCase,
            "lowercase" => CaseStyle::LowerCase,
            "UPPERCASE" => CaseStyle::UpperCase,
            _ => panic!(
                "Unexpected case style for serialize_all: `{}`. Valid values are: `{:?}`",
                text,
                [
                    "camelCase",
                    "PascalCase",
                    "kebab-case",
                    "snake_case",
                    "SCREAMING_SNAKE_CASE",
                    "SCREAMING-KEBAB-CASE",
                    "lowercase",
                    "UPPERCASE",
                    "title_case",
                    "mixed_case",
                ]
            ),
        }
    }
}

pub trait CaseStyleHelpers {
    fn convert_case(&self, case_style: Option<CaseStyle>) -> String;
}

impl CaseStyleHelpers for syn::Ident {
    fn convert_case(&self, case_style: Option<CaseStyle>) -> String {
        let ident_string = self.to_string();
        if let Some(case_style) = case_style {
            match case_style {
                CaseStyle::PascalCase => ident_string.to_camel_case(),
                CaseStyle::KebabCase => ident_string.to_kebab_case(),
                CaseStyle::MixedCase => ident_string.to_mixed_case(),
                CaseStyle::ShoutySnakeCase => ident_string.to_shouty_snake_case(),
                CaseStyle::SnakeCase => ident_string.to_snake_case(),
                CaseStyle::TitleCase => ident_string.to_title_case(),
                CaseStyle::UpperCase => ident_string.to_uppercase(),
                CaseStyle::LowerCase => ident_string.to_lowercase(),
                CaseStyle::ScreamingKebabCase => ident_string.to_kebab_case().to_uppercase(),
                CaseStyle::CamelCase => {
                    let camel_case = ident_string.to_camel_case();
                    let mut pascal = String::with_capacity(camel_case.len());
                    let mut it = camel_case.chars();
                    if let Some(ch) = it.next() {
                        pascal.extend(ch.to_lowercase());
                    }
                    pascal.extend(it);
                    pascal
                }
            }
        } else {
            ident_string
        }
    }
}

#[test]
fn test_convert_case() {
    let id = syn::Ident::new("test_me", proc_macro2::Span::call_site());
    assert_eq!("testMe", id.convert_case(Some(CaseStyle::CamelCase)));
    assert_eq!("TestMe", id.convert_case(Some(CaseStyle::PascalCase)));
}
