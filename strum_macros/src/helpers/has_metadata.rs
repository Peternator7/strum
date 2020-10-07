use syn::{Attribute, DeriveInput, Meta, NestedMeta, Variant};

/// Represents a type that can have strum metadata associated with it.
pub trait HasMetadata {
    /// Get all the metadata associated with a specific "tag".
    /// All of strum's metadata is nested inside a path such as
    /// #[strum(...)] so this let's us quickly filter down to only our metadata.
    fn get_metadata(&self, ident: &str) -> syn::Result<Vec<Meta>>;
}

fn get_metadata_inner<'a>(
    ident: &str,
    it: impl IntoIterator<Item = &'a Attribute>,
) -> syn::Result<Vec<Meta>> {
    let mut res = Vec::new();

    for attr in it {
        if !attr.path.is_ident(ident) {
            continue;
        }

        let meta = attr.parse_meta()?;
        let nested = match meta {
            Meta::List(syn::MetaList { nested, .. }) => nested,
            _ => {
                return Err(syn::Error::new_spanned(
                    meta,
                    "unrecognized strum attribute form",
                ))
            }
        };

        for nested_meta in nested {
            match nested_meta {
                NestedMeta::Meta(meta) => res.push(meta),
                NestedMeta::Lit(lit) => {
                    return Err(syn::Error::new_spanned(lit, "unexpected literal"))
                }
            }
        }
    }

    Ok(res)
}

impl HasMetadata for Variant {
    fn get_metadata(&self, ident: &str) -> syn::Result<Vec<Meta>> {
        get_metadata_inner(ident, &self.attrs)
    }
}

impl HasMetadata for DeriveInput {
    fn get_metadata(&self, ident: &str) -> syn::Result<Vec<Meta>> {
        get_metadata_inner(ident, &self.attrs)
    }
}
