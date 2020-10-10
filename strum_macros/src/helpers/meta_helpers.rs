use syn::{Lit, Meta, MetaList, MetaNameValue, NestedMeta, Path};

pub trait MetaHelpers {
    fn expect_metalist(&self, msg: &str) -> syn::Result<&MetaList>;
    fn expect_path(&self, msg: &str) -> syn::Result<&Path>;
    fn expect_namevalue(&self, msg: &str) -> syn::Result<&MetaNameValue>;
}

impl MetaHelpers for Meta {
    fn expect_metalist(&self, msg: &str) -> syn::Result<&MetaList> {
        match self {
            Meta::List(list) => Ok(list),
            _ => Err(syn::Error::new_spanned(self, msg)),
        }
    }

    fn expect_path(&self, msg: &str) -> syn::Result<&Path> {
        match self {
            Meta::Path(path) => Ok(path),
            _ => Err(syn::Error::new_spanned(self, msg)),
        }
    }

    fn expect_namevalue(&self, msg: &str) -> syn::Result<&MetaNameValue> {
        match self {
            Meta::NameValue(pair) => Ok(pair),
            _ => Err(syn::Error::new_spanned(self, msg)),
        }
    }
}

pub trait NestedMetaHelpers {
    fn expect_meta(&self, msg: &str) -> syn::Result<&Meta>;
}

impl NestedMetaHelpers for NestedMeta {
    fn expect_meta(&self, msg: &str) -> syn::Result<&Meta> {
        match self {
            NestedMeta::Meta(m) => Ok(m),
            _ => Err(syn::Error::new_spanned(self, msg)),
        }
    }
}

pub trait LitHelpers {
    fn expect_string(&self, msg: &str) -> syn::Result<String>;
}

impl LitHelpers for Lit {
    fn expect_string(&self, msg: &str) -> syn::Result<String> {
        match self {
            Lit::Str(s) => Ok(s.value()),
            _ => Err(syn::Error::new_spanned(self, msg)),
        }
    }
}
