use syn::{Attribute, Meta};

pub mod case_style;
mod meta_helpers;
mod meta_iterator_helpers;
mod metalist_helpers;

pub use self::case_style::CaseStyleHelpers;
pub use self::meta_helpers::MetaHelpers;
pub use self::meta_iterator_helpers::MetaIteratorHelpers;
pub use self::metalist_helpers::MetaListHelpers;

pub fn extract_meta(attrs: &[Attribute]) -> Vec<Meta> {
    attrs
        .iter()
        .filter_map(|attribute| attribute.parse_meta().ok())
        .collect()
}
