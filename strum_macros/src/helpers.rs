
use syn::{Attribute, Meta};

pub fn extract_meta(attrs: &[Attribute]) -> Vec<Meta> {
    attrs.iter()
        .filter_map(|attribute| attribute.interpret_meta())
        .collect()
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
            },
            _ => None,
        })
        .flat_map(|nested| nested)
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
            },
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
