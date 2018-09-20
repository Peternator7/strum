use heck::{CamelCase, KebabCase, MixedCase, ShoutySnakeCase, SnakeCase, TitleCase};
use syn::{Attribute, Ident, Meta};

use case_style::CaseStyle;

pub fn extract_meta(attrs: &[Attribute]) -> Vec<Meta> {
    attrs
        .iter()
        .filter_map(|attribute| attribute.interpret_meta())
        .collect()
}

/// Returns the `Meta` s that are `List`s, and match the given `attr` name.
///
/// For example, `extract_meta_lists(metas, "strum")` returns both `#[strum(Something)]` and
/// `#[strum(SomethingElse)]` for the following declaration.
///
/// ```rust,ignore
/// #[derive(Debug)]
/// #[strum(Something)]
/// #[strum(SomethingElse)]
/// struct MyStruct {}
/// ```
pub fn extract_meta_lists_refs<'meta>(metas: &[&'meta Meta], attr: &str) -> Vec<&'meta Meta> {
    use syn::NestedMeta;
    metas
        .iter()
        // Get all the attributes with our tag on them.
        .filter_map(|meta| match *meta {
            Meta::List(ref metalist) => {
                if metalist.ident == attr {
                    Some(&metalist.nested)
                } else {
                    None
                }
            }
            _ => None,
        }).flat_map(|nested| nested)
        .filter_map(|nested| match *nested {
            NestedMeta::Meta(ref meta) => Some(meta),
            _ => None,
        }).collect()
}

pub fn extract_meta_lists<'meta>(metas: &'meta [Meta], attr: &str) -> Vec<&'meta Meta> {
    extract_meta_lists_refs(&metas.iter().collect::<Vec<_>>(), attr)
}

pub fn unique_meta_list<'meta>(metas: &'meta [Meta], attr: &str) -> Option<&'meta Meta> {
    let mut curr = extract_meta_lists(&metas, attr);
    if curr.len() > 1 {
        panic!(
            "More than one `{}` attribute found on type, {:?}",
            attr, curr
        );
    }

    curr.pop()
}

/// Returns the `Ident`s from the `Meta::List`s that match the given `attr` name.
///
/// For example, `extract_meta_lists(something_metas, "Something")` returns `Abc`, `Def`, and `Ghi` for
/// the following declaration.
///
/// ```rust,ignore
/// #[derive(Debug)]
/// #[strum(Something(Abc, Def), Something(Ghi))]
/// struct MyStruct {}
/// ```
pub fn extract_meta_idents<'meta>(metas: &[&'meta Meta], attr: &str) -> Vec<&'meta Ident> {
    extract_meta_lists_refs(metas, attr)
        .into_iter()
        .filter_map(|meta| match *meta {
            Meta::Word(ref ident) => Some(ident),
            _ => None,
        }).collect()
}

pub fn unique_meta_ident<'meta>(metas: &[&'meta Meta], attr: &str) -> Option<&'meta Ident> {
    let mut curr = extract_meta_idents(metas, attr);
    if curr.len() > 1 {
        panic!(
            "More than one `{}` attribute found on type: {:?}",
            attr, curr
        );
    }

    curr.pop()
}

pub fn extract_attrs(meta: &[Meta], attr: &str, prop: &str) -> Vec<String> {
    use syn::{Lit, MetaNameValue, NestedMeta};
    meta.iter()
        // Get all the attributes with our tag on them.
        .filter_map(|meta| match *meta {
            Meta::List(ref metalist) => {
                if metalist.ident == attr {
                    Some(&metalist.nested)
                } else {
                    None
                }
            }
            _ => None,
        }).flat_map(|nested| nested)
        // Get all the inner elements as long as they start with ser.
        .filter_map(|meta| match *meta {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                ref ident,
                lit: Lit::Str(ref s),
                ..
            })) => {
                if ident == prop {
                    Some(s.value())
                } else {
                    None
                }
            }
            _ => None,
        }).collect()
}

pub fn unique_attr(attrs: &[Meta], attr: &str, prop: &str) -> Option<String> {
    let mut curr = extract_attrs(attrs, attr, prop);
    if curr.len() > 1 {
        panic!("More than one property: {} found on variant", prop);
    }

    curr.pop()
}

pub fn is_disabled(attrs: &[Meta]) -> bool {
    let v = extract_attrs(attrs, "strum", "disabled");
    match v.len() {
        0 => false,
        1 => v[0] == "true",
        _ => panic!("Can't have multiple values for 'disabled'"),
    }
}

pub fn convert_case(ident: &Ident, case_style: Option<CaseStyle>) -> String {
    let ident_string = ident.to_string();
    if let Some(case_style) = case_style {
        match case_style {
            CaseStyle::CamelCase => ident_string.to_camel_case(),
            CaseStyle::KebabCase => ident_string.to_kebab_case(),
            CaseStyle::MixedCase => ident_string.to_mixed_case(),
            CaseStyle::ShoutySnakeCase => ident_string.to_shouty_snake_case(),
            CaseStyle::SnakeCase => ident_string.to_snake_case(),
            CaseStyle::TitleCase => ident_string.to_title_case(),
        }
    } else {
        ident_string
    }
}
