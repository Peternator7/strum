
use syn::Attribute;

pub fn extract_attrs(attrs: &[Attribute], attr: &str, prop: &str) -> Vec<String> {
    use syn::{Lit, Meta, MetaNameValue, NestedMeta};
    attrs.iter()
        .filter_map(|attribute| attribute.interpret_meta())
        // Get all the attributes with our tag on them.
        .filter_map(|meta| match meta {
            Meta::List(metalist) => {
                if metalist.ident == attr {
                    Some(metalist.nested)
                } else {
                    None
                }
            },
            _ => None,
        })
        .flat_map(|nested| nested)
        // Get all the inner elements as long as they start with ser.
        .filter_map(|meta| match meta {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit: Lit::Str(s), .. })) => {
                if ident == prop {
                    Some(s.value())
                } else {
                    None
                }
            },
            _ => None,
        })
        .collect()
}

pub fn unique_attr(attrs: &[Attribute], attr: &str, prop: &str) -> Option<String> {
    let mut curr = extract_attrs(attrs, attr, prop);
    if curr.len() > 1 {
        panic!("More than one property: {} found on variant", prop);
    }

    curr.pop()
}

pub fn is_disabled(attrs: &[Attribute]) -> bool {
    let v = extract_attrs(attrs, "strum", "disabled");
    match v.len() {
        0 => false,
        1 => v[0] == "true",
        _ => panic!("Can't have multiple values for 'disabled'"),
    }
}
