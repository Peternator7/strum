use proc_macro2::TokenStream;
use quote::quote;
use std::default::Default;
use syn::{DeriveInput, Ident, Path, Visibility};

use super::case_style::CaseStyle;
use super::metadata::{DeriveInputExt, EnumDiscriminantsMeta, EnumMeta};
use super::occurrence_error;

pub trait HasTypeProperties {
    fn get_type_properties(&self) -> syn::Result<StrumTypeProperties>;
}

#[derive(Debug, Clone, Default)]
pub struct StrumTypeProperties {
    pub case_style: Option<CaseStyle>,
    pub discriminant_derives: Vec<Path>,
    pub discriminant_name: Option<Ident>,
    pub discriminant_others: Vec<TokenStream>,
    pub discriminant_vis: Option<Visibility>,
}

impl HasTypeProperties for DeriveInput {
    fn get_type_properties(&self) -> syn::Result<StrumTypeProperties> {
        let mut output = StrumTypeProperties::default();

        let strum_meta = self.get_metadata()?;
        let discriminants_meta = self.get_discriminants_metadata()?;

        let mut serialize_all_kw = None;
        for meta in strum_meta {
            match meta {
                EnumMeta::SerializeAll { case_style, kw } => {
                    if let Some(fst_kw) = serialize_all_kw {
                        return Err(occurrence_error(fst_kw, kw, "serialize_all"));
                    }

                    serialize_all_kw = Some(kw);
                    output.case_style = Some(case_style);
                }
            }
        }

        let mut name_kw = None;
        let mut vis_kw = None;
        for meta in discriminants_meta {
            match meta {
                EnumDiscriminantsMeta::Derive { paths, .. } => {
                    output.discriminant_derives.extend(paths);
                }
                EnumDiscriminantsMeta::Name { name, kw } => {
                    if let Some(fst_kw) = name_kw {
                        return Err(occurrence_error(fst_kw, kw, "name"));
                    }

                    name_kw = Some(kw);
                    output.discriminant_name = Some(name);
                }
                EnumDiscriminantsMeta::Vis { vis, kw } => {
                    if let Some(fst_kw) = vis_kw {
                        return Err(occurrence_error(fst_kw, kw, "vis"));
                    }

                    vis_kw = Some(kw);
                    output.discriminant_vis = Some(vis);
                }
                EnumDiscriminantsMeta::Other { path, nested } => {
                    output.discriminant_others.push(quote! { #path(#nested) });
                }
            }
        }

        Ok(output)
    }
}
