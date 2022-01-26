use strum::EnumMetadata;
// To check that from_repr impls on both the enum type,
// and EnumMetadata don't clash in any way.
use strum::FromRepr;

#[derive(Debug, Eq, PartialEq, EnumMetadata, FromRepr)]
#[repr(u8)]
enum ABC {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
}

#[test]
fn abc_variant_names() {
    assert_eq!(ABC::VARIANTS, ["A", "B", "C"]);
}

#[test]
fn abc_variant_count() {
    assert_eq!(ABC::COUNT, 3);
}

#[test]
fn abc_from_repr_same() {
    assert_eq!(
        ABC::from_repr(ABC::A as u8),
        <ABC as EnumMetadata>::from_repr(ABC::A.to_repr())
    )
}
