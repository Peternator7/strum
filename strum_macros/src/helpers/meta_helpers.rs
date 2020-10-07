use syn::{Meta, MetaList, NestedMeta};

pub trait MetaHelpers {
    fn expect_metalist(&self, msg: &str) -> syn::Result<&MetaList>;
    fn expect_path(&self, msg: &str) -> syn::Result<&syn::Path>;
    fn expect_namevalue(&self, msg: &str) -> syn::Result<&syn::MetaNameValue>;
}

impl MetaHelpers for syn::Meta {
    fn expect_metalist(&self, msg: &str) -> syn::Result<&MetaList> {
        match self {
            Meta::List(list) => Ok(list),
            _ => Err(syn::Error::new_spanned(self, msg)),
        }
    }

    fn expect_path(&self, msg: &str) -> syn::Result<&syn::Path> {
        match self {
            Meta::Path(path) => Ok(path),
            _ => Err(syn::Error::new_spanned(self, msg)),
        }
    }

    fn expect_namevalue(&self, msg: &str) -> syn::Result<&syn::MetaNameValue> {
        match self {
            Meta::NameValue(pair) => Ok(pair),
            _ => Err(syn::Error::new_spanned(self, msg)),
        }
    }
}

pub trait NestedMetaHelpers {
    fn expect_meta(&self, msg: &str) -> syn::Result<&syn::Meta>;
    fn expect_lit(&self, msg: &str) -> syn::Result<&syn::Lit>;
}

impl NestedMetaHelpers for NestedMeta {
    fn expect_meta(&self, msg: &str) -> syn::Result<&Meta> {
        match self {
            syn::NestedMeta::Meta(m) => Ok(m),
            _ => Err(syn::Error::new_spanned(self, msg)),
        }
    }
    fn expect_lit(&self, msg: &str) -> syn::Result<&syn::Lit> {
        match self {
            syn::NestedMeta::Lit(l) => Ok(l),
            _ => Err(syn::Error::new_spanned(self, msg)),
        }
    }
}

pub trait LitHelpers {
    fn expect_string(&self, msg: &str) -> syn::Result<String>;
}

impl LitHelpers for syn::Lit {
    fn expect_string(&self, msg: &str) -> syn::Result<String> {
        match self {
            syn::Lit::Str(s) => Ok(s.value()),
            _ => Err(syn::Error::new_spanned(self, msg)),
        }
    }
}
