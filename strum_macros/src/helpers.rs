use heck::{CamelCase, KebabCase, MixedCase, ShoutySnakeCase, SnakeCase, TitleCase};
use syn::{Attribute, Ident, Meta, MetaList, Path};

use case_style::CaseStyle;

pub fn extract_meta(attrs: &[Attribute]) -> Vec<Meta> {
    attrs
        .iter()
        .flat_map(|attribute| attribute.parse_meta())
        .collect()
}

pub fn filter_meta_lists<'meta, MetaIt, F>(
    metas: MetaIt,
    filter: F,
) -> impl Iterator<Item = &'meta MetaList>
where
    MetaIt: Iterator<Item = &'meta Meta>,
    F: Fn(&MetaList) -> bool,
{
    metas.filter_map(move |meta| match meta {
        Meta::List(metalist) => {
            if filter(metalist) {
                Some(metalist)
            } else {
                None
            }
        }
        _ => None,
    })
}

/// Returns the `MetaList`s with the given attr name.
///
/// For example, `get_meta_list(type_meta.iter(), "strum_discriminant")` for the following snippet
/// will return an iterator with `#[strum_discriminant(derive(EnumIter))]` and
/// `#[strum_discriminant(name(MyEnumVariants))]`.
///
/// ```rust,ignore
/// #[derive(Debug)]
/// #[strum_discriminant(derive(EnumIter))]
/// #[strum_discriminant(name(MyEnumVariants))]
/// enum MyEnum { A }
/// ```
pub fn get_meta_list<'iter, 'meta: 'iter, MetaIt>(
    metas: MetaIt,
    attr: &'iter str,
) -> impl Iterator<Item = &'meta MetaList> + 'iter
where
    MetaIt: Iterator<Item = &'meta Meta> + 'iter,
{
    filter_meta_lists(metas, move |metalist| eq_path_str(&metalist.path, attr))
}

/// Returns the `MetaList` that matches the given name from the list of `Meta`s, or `None`.
///
/// # Panics
///
/// Panics if more than one `Meta` exists with the name.
pub fn unique_meta_list<'meta, MetaIt>(metas: MetaIt, attr: &'_ str) -> Option<&'meta MetaList>
where
    MetaIt: Iterator<Item = &'meta Meta>,
{
    let mut curr = get_meta_list(metas, attr).collect::<Vec<_>>();
    if curr.len() > 1 {
        panic!("More than one `{}` attribute found on type", attr);
    }

    curr.pop()
}

/// Returns an iterator of the `Meta`s from the given `MetaList`.
pub fn extract_list_metas(metalist: &MetaList) -> impl Iterator<Item = &Meta> {
    use syn::NestedMeta;
    metalist.nested.iter().filter_map(|nested| match nested {
        NestedMeta::Meta(meta) => Some(meta),
        _ => None,
    })
}

/// Returns the `Ident` of the `Meta::Word`, or `None`.
pub fn get_meta_ident(meta: &Meta) -> Option<&Ident> {
    match meta {
        Meta::Path(path) => Some(&path.segments[0].ident),
        _ => None,
    }
}

pub fn extract_attrs(meta: &[Meta], attr: &str, prop: &str) -> Vec<String> {
    use syn::{Lit, MetaNameValue, NestedMeta};
    meta.iter()
        // Get all the attributes with our tag on them.
        .filter_map(|meta| match meta {
            Meta::List(metalist) => {
                if eq_path_str(&metalist.path, attr) {
                    Some(&metalist.nested)
                } else {
                    None
                }
            }
            _ => None,
        })
        .flat_map(|nested| nested)
        // Get all the inner elements as long as they start with ser.
        .filter_map(|meta| match meta {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(s),
                ..
            })) => {
                if eq_path_str(path, prop) {
                    Some(s.value())
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
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

/// Checks whether the path is equal to the given string.
///
/// Returns `true` if they are same.
///
/// Note that the given string should be a single path segment.
/// In other words, it should not be multi-segment path like `a::b::c`.
pub fn eq_path_str(path: &Path, s: &str) -> bool {
    (path.segments.len() == 1) && (path.segments[0].ident == s)
}
