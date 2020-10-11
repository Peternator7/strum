use proc_macro2::TokenStream;
use quote::quote;
use std::default::Default;
use syn::{DeriveInput, Ident, Path};

use crate::helpers::case_style::CaseStyle;
use crate::helpers::metadata::{DeriveInputExt, EnumDiscriminantsMeta, EnumMeta};

pub trait HasTypeProperties {
    fn get_type_properties(&self) -> syn::Result<StrumTypeProperties>;
}

#[derive(Debug, Clone, Default)]
pub struct StrumTypeProperties {
    pub case_style: Option<CaseStyle>,
    pub discriminant_derives: Vec<Path>,
    pub discriminant_name: Option<Ident>,
    pub discriminant_others: Vec<TokenStream>,
}

impl HasTypeProperties for DeriveInput {
    fn get_type_properties(&self) -> syn::Result<StrumTypeProperties> {
        let mut output = StrumTypeProperties::default();

        let strum_meta = self.get_metadata()?;
        let discriminants_meta = self.get_discriminants_metadata()?;

        for meta in strum_meta {
            match meta {
                EnumMeta::SerializeAll { case_style, .. } => {
                    if output.case_style.is_some() {
                        panic!("found multiple values of serialize_all");
                    }

                    output.case_style = Some(case_style);
                }
            }
        }

        for meta in discriminants_meta {
            match meta {
                EnumDiscriminantsMeta::Derive { paths, .. } => {
                    output.discriminant_derives.extend(paths);
                }
                EnumDiscriminantsMeta::Name { name, .. } => {
                    if output.discriminant_name.is_some() {
                        panic!("multiple occurrences of 'name'");
                    }

                    output.discriminant_name = Some(name);
                }
                EnumDiscriminantsMeta::Other { path, nested } => {
                    output.discriminant_others.push(quote! { #path(#nested) });
                }
            }
        }

        Ok(output)
    }
}
