pub use self::case_style::CaseStyleHelpers;
pub use self::type_props::HasTypeProperties;
pub use self::variant_props::HasStrumVariantProperties;

pub mod case_style;
mod metadata;
pub mod type_props;
pub mod variant_props;

use heck::ToSnakeCase;
use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;

pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
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

/// heck doesn't treat numbers as new words, but this function does.
/// E.g. for input `Hello2You`, heck would output `hello2_you`, and snakify would output `hello_2_you`.
pub fn snakify(s: &str) -> String {
    let mut output: Vec<char> = s.to_string().to_snake_case().chars().collect();
    let mut num_starts = vec![];
    for (pos, c) in output.iter().enumerate() {
        if c.is_digit(10) && pos != 0 && !output[pos - 1].is_digit(10) {
            num_starts.push(pos);
        }
    }
    // need to do in reverse, because after inserting, all chars after the point of insertion are off
    for i in num_starts.into_iter().rev() {
        output.insert(i, '_')
    }
    output.into_iter().collect()
}
