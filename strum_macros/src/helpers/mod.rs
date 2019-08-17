
use syn::{Attribute, Meta};

pub mod case_style;
mod metalist_helpers;
mod meta_helpers;
mod meta_iterator_helpers;

pub use self::metalist_helpers::MetaListHelpers;
pub use self::meta_helpers::MetaHelpers;
pub use self::meta_iterator_helpers::MetaIteratorHelpers;
pub use self::case_style::CaseStyleHelpers;

pub fn extract_meta(attrs: &[Attribute]) -> Vec<Meta> {
    attrs
        .iter()
        .filter_map(|attribute| attribute.parse_meta().ok())
        .collect()
}
