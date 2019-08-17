use syn::{Meta, MetaList};

use super::MetaHelpers;
use super::MetaListHelpers;

pub trait MetaIteratorHelpers {
    type Item;

    fn unique_meta_list(self, attr: &str) -> Option<Self::Item>;
    fn extract_attrs(self, attr: &str, prop: &str) -> Vec<String>;

    fn unique_attr(self, attr: &str, prop: &str) -> Option<String> 
        where Self: Sized
    {
        let mut curr = self.extract_attrs(attr, prop);
        if curr.len() > 1 {
            panic!("More than one property: {} found on variant", prop);
        }

        curr.pop()
    }

    fn is_disabled(self) -> bool 
        where Self: Sized
    {
        let v = self.extract_attrs("strum", "disabled");
        match v.len() {
            0 => false,
            1 => v[0] == "true",
            _ => panic!("Can't have multiple values for 'disabled'"),
        }
    }
}

impl <'a, I> MetaIteratorHelpers for I 
where
    I: std::iter::IntoIterator<Item=&'a Meta>
{
    type Item=&'a MetaList;

    /// Returns the `MetaList` that matches the given name from the list of `Meta`s, or `None`.
    ///
    /// # Panics
    ///
    /// Panics if more than one `Meta` exists with the name.
    fn unique_meta_list(self, attr: &str) -> Option<&'a MetaList>
    where
        Self: Sized
    {
        // let mut curr = get_meta_list(metas.into_iter(), attr).collect::<Vec<_>>();
        let mut curr = self.into_iter()
            .filter_map(|meta| meta.try_metalist())
            .filter(|list| list.path.is_ident(attr))
            .collect::<Vec<_>>();

        if curr.len() > 1 {
            panic!("More than one `{}` attribute found on type", attr);
        }

        curr.pop()
    }

    fn extract_attrs(self, attr: &str, prop: &str) -> Vec<String> 
    where
        Self: Sized
    {
        use syn::{Lit, MetaNameValue};
        self.into_iter()
            .filter_map(|meta| meta.try_metalist())
            .filter(|list| list.path.is_ident(attr))
            .flat_map(|list| list.expand_inner())
            // Get all the inner elements as long as they start with ser.
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