use super::metadata::{InnerVariantExt, InnerVariantMeta};
use syn::{Field, LitStr};

pub trait HasInnerVariantProperties {
    fn get_variant_inner_properties(&self) -> syn::Result<StrumInnerVariantProperties>;
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct StrumInnerVariantProperties {
    pub default_with: Option<LitStr>,
}

impl HasInnerVariantProperties for Field {
    fn get_variant_inner_properties(&self) -> syn::Result<StrumInnerVariantProperties> {
        let mut output = StrumInnerVariantProperties { default_with: None };

        for meta in self.get_named_metadata()? {
            match meta {
                InnerVariantMeta::DefaultWith { kw: _, value } => {
                    output.default_with = Some(value);
                }
            }
        }

        Ok(output)
    }
}
