use std::convert::From;
use std::default::Default;

use crate::helpers::case_style::CaseStyle;
use crate::helpers::has_metadata::HasMetadata;
use crate::helpers::{MetaHelpers, NestedMetaHelpers};

pub trait HasTypeProperties {
    fn get_type_properties(&self) -> StrumTypeProperties;
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct StrumTypeProperties {
    pub case_style: Option<CaseStyle>,
    pub discriminant_derives: Vec<syn::Path>,
    pub discriminant_name: Option<syn::Path>,
    pub discriminant_others: Vec<syn::Meta>,
}

impl HasTypeProperties for syn::DeriveInput {
    fn get_type_properties(&self) -> StrumTypeProperties {
        let mut output = StrumTypeProperties::default();

        let strum_meta = self.get_metadata("strum");
        let discriminants_meta = self.get_metadata("strum_discriminants");

        for meta in strum_meta {
            let meta = match meta {
                syn::Meta::NameValue(mv) => mv,
                _ => panic!("strum on types only supports key-values"),
            };

            if meta.path.is_ident("serialize_all") {
                let style = match meta.lit {
                    syn::Lit::Str(s) => s.value(),
                    _ => panic!("expected string value for 'serialize_all'"),
                };

                if output.case_style.is_some() {
                    panic!("found multiple values of serialize_all");
                }

                output.case_style = Some(CaseStyle::from(&*style));
            } else {
                panic!("unrecognized attribue found on strum(..)");
            }
        }

        for meta in discriminants_meta {
            match meta {
                syn::Meta::List(ref ls) => {
                    if ls.path.is_ident("derive") {
                        let paths = ls
                            .nested
                            .iter()
                            .map(|meta| meta.expect_meta("unexpected literal").path().clone());

                        output.discriminant_derives.extend(paths);
                    } else if ls.path.is_ident("name") {
                        if ls.nested.len() != 1 {
                            panic!("name expects exactly 1 value");
                        }

                        let value = ls.nested.first().expect("unexpected error");
                        let name = value
                            .expect_meta("unexpected literal")
                            .expect_path("name must be an identifier");

                        if output.discriminant_name.is_some() {
                            panic!("multiple occurrences of 'name'");
                        }

                        output.discriminant_name = Some(name.clone());
                    } else {
                        output.discriminant_others.push(meta);
                    }
                }
                _ => {
                    output.discriminant_others.push(meta);
                }
            }
        }

        output
    }
}
