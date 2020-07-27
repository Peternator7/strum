use syn::{Meta, MetaList, NestedMeta};

pub trait MetaHelpers {
    fn expect_metalist(&self, msg: &str) -> &MetaList;
    fn expect_path(&self, msg: &str) -> &syn::Path;
    fn expect_namevalue(&self, msg: &str) -> &syn::MetaNameValue;
}

impl MetaHelpers for syn::Meta {
    fn expect_metalist(&self, msg: &str) -> &MetaList {
        match self {
            Meta::List(list) => list,
            _ => panic!("{}", msg),
        }
    }

    fn expect_path(&self, msg: &str) -> &syn::Path {
        match self {
            Meta::Path(path) => path,
            _ => panic!("{}", msg),
        }
    }

    fn expect_namevalue(&self, msg: &str) -> &syn::MetaNameValue {
        match self {
            Meta::NameValue(pair) => pair,
            _ => panic!("{}", msg),
        }
    }
}

pub trait NestedMetaHelpers {
    fn expect_meta(&self, msg: &str) -> &syn::Meta;
    fn expect_lit(&self, msg: &str) -> &syn::Lit;
}

impl NestedMetaHelpers for NestedMeta {
    fn expect_meta(&self, msg: &str) -> &Meta {
        match self {
            syn::NestedMeta::Meta(m) => m,
            _ => panic!("{}", msg),
        }
    }
    fn expect_lit(&self, msg: &str) -> &syn::Lit {
        match self {
            syn::NestedMeta::Lit(l) => l,
            _ => panic!("{}", msg),
        }
    }
}

pub trait LitHelpers {
    fn expect_string(&self, msg: &str) -> String;
}

impl LitHelpers for syn::Lit {
    fn expect_string(&self, msg: &str) -> String {
        match self {
            syn::Lit::Str(s) => s.value(),
            _ => panic!("{}", msg),
        }
    }
}
