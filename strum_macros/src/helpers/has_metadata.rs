///Represents a type that can have strum metadata associated with it.
pub trait HasMetadata {
    /// Get all the metadata associated with a specific "tag".
    /// All of strum's metadata is nested inside a path such as
    /// #[strum(...)] so this let's us quickly filter down to only our metadata.
    fn get_metadata(&self, ident: &str) -> Vec<syn::Meta>;
}

fn get_metadata_inner<'a>(
    ident: &str,
    it: impl IntoIterator<Item = &'a syn::Attribute>,
) -> Vec<syn::Meta> {
    it.into_iter()
        .filter(|attr| attr.path.is_ident(ident))
        .map(|attr| attr.parse_meta().unwrap())
        .filter_map(|meta| match meta {
            syn::Meta::List(syn::MetaList { path, nested, .. }) => {
                if path.is_ident(ident) {
                    Some(nested)
                } else {
                    None
                }
            }
            _ => None,
        })
        .flat_map(|id| id)
        .map(|nested| match nested {
            syn::NestedMeta::Meta(meta) => meta,
            _ => panic!("unexpected literal parsing strum attributes"),
        })
        .collect()
}

impl HasMetadata for syn::Variant {
    fn get_metadata(&self, ident: &str) -> Vec<syn::Meta> {
        get_metadata_inner(ident, &self.attrs)
    }
}

impl HasMetadata for syn::DeriveInput {
    fn get_metadata(&self, ident: &str) -> Vec<syn::Meta> {
        get_metadata_inner(ident, &self.attrs)
    }
}
