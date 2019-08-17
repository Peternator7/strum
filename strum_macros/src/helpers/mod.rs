use heck::{CamelCase, KebabCase, MixedCase, ShoutySnakeCase, SnakeCase, TitleCase};
use syn::{Attribute, Ident, Meta, MetaList};
use crate::helpers::case_style::CaseStyle;

pub mod case_style;
mod metalist_helpers;
mod meta_helpers;
mod meta_iterator_helpers;

pub use self::metalist_helpers::MetaListHelpers;
pub use self::meta_helpers::MetaHelpers;
pub use self::meta_iterator_helpers::MetaIteratorHelpers;

pub fn extract_meta(attrs: &[Attribute]) -> Vec<Meta> {
    attrs
        .iter()
        .filter_map(|attribute| attribute.parse_meta().ok())
        .collect()
}

/// Returns the `MetaList` that matches the given name from the list of `Meta`s, or `None`.
///
/// # Panics
///
/// Panics if more than one `Meta` exists with the name.
pub fn unique_meta_list<'meta, MetaIt>(metas: MetaIt, attr: &'meta str) -> Option<&'meta MetaList>
where
    MetaIt: Iterator<Item = &'meta Meta>,
{
    // let mut curr = get_meta_list(metas.into_iter(), attr).collect::<Vec<_>>();
    let mut curr = metas
        .filter_map(|meta| meta.try_metalist())
        .filter(|list| list.path.is_ident(attr))
        .collect::<Vec<_>>();

    if curr.len() > 1 {
        panic!("More than one `{}` attribute found on type", attr);
    }

    curr.pop()
}

pub fn convert_case(ident: &Ident, case_style: Option<CaseStyle>) -> String {
    let ident_string = ident.to_string();
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

#[test]
fn test_convert_case() {
    let id = Ident::new("test_me", proc_macro2::Span::call_site());
    assert_eq!("testMe", convert_case(&id, Some(CaseStyle::CamelCase)));
    assert_eq!("TestMe", convert_case(&id, Some(CaseStyle::PascalCase)));
}
