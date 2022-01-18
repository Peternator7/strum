#![cfg(test)]
use strum::EnumMaskIter;
use strum::{EnumRepr, OpaqueRepr};
use strum_macros::EnumMask;

#[derive(Clone, Debug, Eq, PartialEq, EnumMask)]
#[repr(u8)]
enum ABC {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
}

#[test]
fn mask_iter() {
    let mut mask = ABC::A.opaque();
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::A]);
    let mut mask = ABC::A.opaque() | ABC::B;
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::A, ABC::B]);
    let mut mask = ABC::C.opaque() | ABC::B.opaque();
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::B, ABC::C]);
}
