pub use self::case_style::snakify;
pub use self::inner_variant_props::HasInnerVariantProperties;
pub use self::type_props::HasTypeProperties;
pub use self::variant_props::HasStrumVariantProperties;

pub mod case_style;
pub mod inner_variant_props;
mod metadata;
pub mod type_props;
pub mod variant_props;

use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Fields, FieldsUnnamed, Type, Variant};

pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
}

pub fn non_unit_variant_error() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "This macro only supports enums of strictly unit variants. Consider \
        using it in conjunction with [`EnumDiscriminants`]"
    )
}

pub fn non_new_type_variant_error(additional_info: &str) -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        format!(
            "This macro only supports enums of strictly new type variants, but {additional_info}"
        ),
    )
}

pub fn no_associated_deref_type_specified() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "expected a deref target specified via attribute, e.g. #[strum_deref_target(T)]",
    )
}

pub fn strum_discriminants_passthrough_error(span: &impl Spanned) -> syn::Error {
    syn::Error::new(
        span.span(),
        "expected a pass-through attribute, e.g. #[strum_discriminants(serde(rename = \"var0\"))]",
    )
}

pub fn occurrence_error<T: ToTokens>(fst: T, snd: T, attr: &str) -> syn::Error {
    let mut e = syn::Error::new_spanned(
        snd,
        format!("Found multiple occurrences of strum({})", attr),
    );
    e.combine(syn::Error::new_spanned(fst, "first one here"));
    e
}

pub fn get_new_type_variant(enum_variant: &Variant) -> syn::Result<(Ident, Type)> {
    match &enum_variant.fields {
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            if unnamed.len() != 1 {
                Err(non_new_type_variant_error(
                    "the list of type parameters is different from 1",
                ))
            } else if let Some(new_type) = unnamed.first() {
                Ok((enum_variant.ident.clone(), new_type.ty.clone()))
            } else {
                unreachable!("`unnamed.len()` must be 1 in the previous branch, so `.first()` should not return `None`");
            }
        }
        _ => Err(non_new_type_variant_error(&format!(
            "the variant {} is not a tuple-struct",
            enum_variant.ident
        ))),
    }
}
