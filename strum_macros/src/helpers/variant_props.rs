use std::collections::HashMap;
use std::default::Default;

use crate::helpers::case_style::{CaseStyle, CaseStyleHelpers};
use crate::helpers::has_metadata::HasMetadata;
use crate::helpers::{LitHelpers, MetaHelpers, NestedMetaHelpers};

pub trait HasStrumVariantProperties {
    fn get_variant_properties(&self) -> StrumVariantProperties;
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct StrumVariantProperties {
    pub is_disabled: bool,
    pub default: bool,
    pub message: Option<String>,
    pub detailed_message: Option<String>,
    pub string_props: HashMap<String, String>,
    serialize: Vec<String>,
    to_string: Option<String>,
    ident: Option<syn::Ident>,
}

impl StrumVariantProperties {
    pub fn get_preferred_name(&self, case_style: Option<CaseStyle>) -> String {
        if let Some(ref to_string) = self.to_string {
            to_string.clone()
        } else {
            let mut serialized = self.serialize.clone();
            serialized.sort_by_key(|s| s.len());
            if let Some(n) = serialized.pop() {
                n
            } else {
                self.ident
                    .as_ref()
                    .expect("identifier")
                    .convert_case(case_style)
            }
        }
    }

    pub fn get_serializations(&self, case_style: Option<CaseStyle>) -> Vec<String> {
        let mut attrs = self.serialize.clone();
        if let Some(ref to_string) = self.to_string {
            attrs.push(to_string.clone());
        }

        if attrs.is_empty() {
            attrs.push(
                self.ident
                    .as_ref()
                    .expect("identifier")
                    .convert_case(case_style),
            );
        }

        attrs
    }
}

impl HasStrumVariantProperties for syn::Variant {
    fn get_variant_properties(&self) -> StrumVariantProperties {
        let mut output = StrumVariantProperties::default();
        output.ident = Some(self.ident.clone());

        for meta in self.get_metadata("strum") {
            match meta {
                syn::Meta::NameValue(syn::MetaNameValue { path, lit, .. }) => {
                    if path.is_ident("message") {
                        if output.message.is_some() {
                            panic!("message is set twice on the same variant");
                        }

                        output.message = Some(lit.expect_string("expected string"));
                    } else if path.is_ident("detailed_message") {
                        if output.detailed_message.is_some() {
                            panic!("detailed message set twice on the same variant");
                        }

                        output.detailed_message = Some(lit.expect_string("expected string"));
                    } else if path.is_ident("serialize") {
                        output.serialize.push(lit.expect_string("expected string"));
                    } else if path.is_ident("to_string") {
                        if output.to_string.is_some() {
                            panic!("to_string is set twice on the same variant");
                        }

                        output.to_string = Some(lit.expect_string("expected string"));
                    } else if path.is_ident("disabled") {
                        panic!("this method is deprecated. Prefer #[strum(disabled)] instead of #[strum(disabled=\"true\")]");
                    } else if path.is_ident("default") {
                        panic!("this method is deprecated. Prefer #[strum(default)] instead of #[strum(default=\"true\")]");
                    } else {
                        panic!("unrecognized value in strum(..) attribute");
                    }
                }
                syn::Meta::Path(p) => {
                    if p.is_ident("disabled") {
                        output.is_disabled = true;
                    } else if p.is_ident("default") {
                        output.default = true;
                    } else {
                        panic!("unrecognized value in strum(..) attribute");
                    }
                }
                syn::Meta::List(syn::MetaList { path, nested, .. }) => {
                    if path.is_ident("props") {
                        for p in nested {
                            let p = p
                                .expect_meta("unexpected literal found in props")
                                .expect_namevalue("props must be key-value pairs");

                            let key = p
                                .path
                                .get_ident()
                                .expect("key must be an identifier")
                                .to_string();

                            let value = p.lit.expect_string("expected string");
                            output.string_props.insert(key, value);
                        }
                    } else {
                        panic!("unrecognized value in strum(..) attribute");
                    }
                }
            }
        }

        output
    }
}
