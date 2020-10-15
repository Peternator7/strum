use std::default::Default;
use syn::{Ident, LitStr, Variant};

use crate::helpers::case_style::{CaseStyle, CaseStyleHelpers};
use crate::helpers::metadata::{VariantExt, VariantMeta};

pub trait HasStrumVariantProperties {
    fn get_variant_properties(&self) -> syn::Result<StrumVariantProperties>;
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct StrumVariantProperties {
    pub is_disabled: bool,
    pub default: bool,
    pub message: Option<LitStr>,
    pub detailed_message: Option<LitStr>,
    pub string_props: Vec<(LitStr, LitStr)>,
    serialize: Vec<LitStr>,
    to_string: Option<LitStr>,
    ident: Option<Ident>,
}

impl StrumVariantProperties {
    fn ident_as_str(&self, case_style: Option<CaseStyle>) -> LitStr {
        let ident = self.ident.as_ref().expect("identifier");
        LitStr::new(&ident.convert_case(case_style), ident.span())
    }

    pub fn get_preferred_name(&self, case_style: Option<CaseStyle>) -> LitStr {
        if let Some(to_string) = &self.to_string {
            to_string.clone()
        } else {
            let mut serialized = self.serialize.clone();
            serialized.sort_by_key(|s| s.value().len());
            if let Some(n) = serialized.pop() {
                n
            } else {
                self.ident_as_str(case_style)
            }
        }
    }

    pub fn get_serializations(&self, case_style: Option<CaseStyle>) -> Vec<LitStr> {
        let mut attrs = self.serialize.clone();
        if let Some(to_string) = &self.to_string {
            attrs.push(to_string.clone());
        }

        if attrs.is_empty() {
            attrs.push(self.ident_as_str(case_style));
        }

        attrs
    }
}

impl HasStrumVariantProperties for Variant {
    fn get_variant_properties(&self) -> syn::Result<StrumVariantProperties> {
        let mut output = StrumVariantProperties::default();
        output.ident = Some(self.ident.clone());

        for meta in self.get_metadata()? {
            match meta {
                VariantMeta::Message { value, .. } => {
                    if output.message.is_some() {
                        panic!("message is set twice on the same variant");
                    }

                    output.message = Some(value);
                }
                VariantMeta::DetailedMessage { value, .. } => {
                    if output.detailed_message.is_some() {
                        panic!("detailed message set twice on the same variant");
                    }

                    output.detailed_message = Some(value);
                }
                VariantMeta::Serialize { value, .. } => {
                    output.serialize.push(value);
                }
                VariantMeta::ToString { value, .. } => {
                    if output.to_string.is_some() {
                        panic!("to_string is set twice on the same variant");
                    }

                    output.to_string = Some(value);
                }
                VariantMeta::Disabled(_) => {
                    output.is_disabled = true;
                }
                VariantMeta::Default(_) => {
                    output.default = true;
                }
                VariantMeta::Props { props, .. } => {
                    output.string_props.extend(props);
                }
            }
        }

        Ok(output)
    }
}
