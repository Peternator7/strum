use strum::EnumMaskIter;
use strum::{EnumRepr, OpaqueRepr};
use strum_macros::EnumMask;

#[derive(Debug, Eq, PartialEq, EnumMask)]
#[repr(u8)]
enum ABC {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
}

#[test]
fn mask_iter() {
    let mask = ABC::A.opaque();
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::A]);
    let mask = ABC::A.opaque() | ABC::B;
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::A, ABC::B]);
    let mask = ABC::C.opaque() | ABC::B.opaque();
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::B, ABC::C]);
}

#[derive(Debug, Eq, PartialEq, EnumMask)]
#[repr(u8)]
enum ReprBoundary {
    End = 1 << 7,
}

#[test]
fn test_repr_boundary() {
    let mask = ReprBoundary::End.opaque();
    assert_eq!(
        mask.mask_iter().collect::<Vec<ReprBoundary>>(),
        [ReprBoundary::End]
    );
}

#[derive(Debug, Eq, PartialEq, EnumMask)]
#[repr(u8)]
enum ReprSaturated {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
    D = 1 << 3,
    E = 1 << 4,
    F = 1 << 5,
    G = 1 << 6,
    H = 1 << 7,
}

#[test]
fn test_repr_saturated() {
    use ReprSaturated::*;
    let mask = A.opaque();
    assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [A]);
    let mask = A.opaque() | H;
    assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [A, H]);
    let mask = B.opaque() | H;
    assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [B, H]);
    let mask = B.opaque() | G;
    assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [B, G]);
    let mask = G.opaque() | B;
    assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [B, G]);
}
