use syn::{Meta, MetaList};

pub trait MetaHelpers {
    fn try_metalist(&self) -> Option<&MetaList>;
    fn try_path(&self) -> Option<&syn::Path>;
    fn try_namevalue(&self) -> Option<&syn::MetaNameValue>;
}

impl MetaHelpers for syn::Meta {
    fn try_metalist(&self) -> Option<&MetaList> {
        match self {
            Meta::List(list) => Some(list),
            _ => None,
        }
    }

    fn try_path(&self) -> Option<&syn::Path> {
        match self {
            Meta::Path(path) => Some(path),
            _ => None,
        }
    }

    fn try_namevalue(&self) -> Option<&syn::MetaNameValue> {
        match self {
            Meta::NameValue(pair) => Some(pair),
            _ => None,
        }
    }
}
