//! Test that derive macro implementations do not emit deprecated or unreachable
//! code warnings.
#![deny(deprecated, unreachable_code)]

use strum_macros::{
    AsRefStr, Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumMessage, EnumProperty,
    EnumString, EnumTable, EnumTryAs, FromRepr, IntoStaticStr, VariantArray,
};

#[derive(
    Debug,
    Display,
    AsRefStr,
    EnumCount,
    EnumDiscriminants,
    EnumIs,
    EnumMessage,
    EnumProperty,
    EnumTryAs,
    IntoStaticStr,
)]
pub enum Color {
    Red,
    Blue {
        hue: usize,
    },
    #[strum(default)]
    Green(f64),
    // Deprecated variant
    #[deprecated]
    Purple(String),
    // Unreachable variant
    Olo(std::convert::Infallible),
}

#[derive(VariantArray, EnumString, EnumIter, EnumTable, FromRepr)]
pub enum Bar {
    A,
    B,
    #[deprecated]
    C,
}
