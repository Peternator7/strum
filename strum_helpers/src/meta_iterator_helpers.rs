use super::MetaHelpers;
use super::MetaListHelpers;
use syn::Meta;

pub trait MetaIteratorHelpers {
    fn find_attribute(&self, attr: &str) -> std::vec::IntoIter<&Meta>;
    fn find_properties(&self, attr: &str, prop: &str) -> Vec<String>;

    fn find_unique_property(&self, attr: &str, prop: &str) -> Option<String> {
        let mut curr = self.find_properties(attr, prop);
        if curr.len() > 1 {
            panic!("More than one property: {} found on variant", prop);
        }

        curr.pop()
    }

    fn is_disabled(&self) -> bool {
        let v = self.find_properties("strum", "disabled");
        match v.len() {
            0 => false,
            1 => v[0] == "true",
            _ => panic!("Can't have multiple values for 'disabled'"),
        }
    }
}

//impl MetaIteratorHelpers for [Meta]
impl<T> MetaIteratorHelpers for [T]
where
    T: std::borrow::Borrow<Meta>,
{
    fn find_attribute(&self, attr: &str) -> std::vec::IntoIter<&Meta> {
        self.iter()
            .filter_map(|meta| meta.borrow().try_metalist())
            .filter(|list| list.path.is_ident(attr))
            .flat_map(|list| list.expand_inner())
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn find_properties(&self, attr: &str, prop: &str) -> Vec<String> {
        use syn::{Lit, MetaNameValue};
        self.iter()
            // Only look at MetaList style attributes `[strum(...)]`
            .filter_map(|meta| meta.borrow().try_metalist())
            .filter(|list| list.path.is_ident(attr))
            .flat_map(|list| list.expand_inner())
            // Match all the properties with a given ident `[strum(serialize = "value")]`
            .filter_map(|meta| match *meta {
                Meta::NameValue(MetaNameValue {
                    ref path,
                    lit: Lit::Str(ref s),
                    ..
                }) => {
                    if path.is_ident(prop) {
                        Some(s.value())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect()
    }
}
