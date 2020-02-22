use syn::Meta;

pub trait MetaListHelpers {
    fn expand_inner(&self) -> Vec<&Meta>;
}

impl MetaListHelpers for syn::MetaList {
    fn expand_inner(&self) -> Vec<&Meta> {
        use syn::NestedMeta;
        self.nested
            .iter()
            .filter_map(|nested| match *nested {
                NestedMeta::Meta(ref meta) => Some(meta),
                _ => None,
            })
            .collect()
    }
}
