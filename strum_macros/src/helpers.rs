
use syn;
use syn::Attribute;

pub fn extract_attrs<'a>(attrs: &'a [Attribute], attr: &str, prop: &str) -> Vec<&'a str> {
    attrs.iter()
        // Get all the attributes with our tag on them.
        .filter_map(|attribute| {
            use syn::MetaItem::*;
            if let List(ref i, ref nested) = attribute.value {
                if i == attr { Some(nested) } else { None }
            } else {
                None
            }
        })
        .flat_map(|nested| nested)
        // Get all the inner elements as long as they start with ser.
        .filter_map(|attribute| {
            use syn::NestedMetaItem::*;
            use syn::MetaItem::*;
            if let &MetaItem(NameValue(ref i, syn::Lit::Str(ref s, ..))) = attribute {
                if i == prop { Some(&**s) } else { None }
            } else {
                None
            }
        }).collect()
}

pub fn unique_attr<'a>(attrs: &'a [Attribute], attr: &str, prop: &str) -> Option<&'a str> {
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