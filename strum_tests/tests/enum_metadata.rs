use strum::EnumMetadata;
// To check that from_repr impls on both the enum type,
// and EnumMetadata don't clash in any way.
use core::ops;
use num_traits::ops::checked;
use num_traits::ops::wrapping;
use num_traits::Num;
use num_traits::PrimInt;
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

// Scaffolding for further tests, and an example
// Iterator type which uses the EnumMetadata trait.
impl<
        R: Copy
            + ops::BitOr
            + ops::BitAnd
            + ops::BitXor
            + ops::Shr
            + ops::Shl
            + ops::Not
            + ops::BitOrAssign
            + ops::BitAndAssign
            + ops::BitXorAssign
            + ops::ShrAssign
            + ops::ShlAssign
            + checked::CheckedShl
            + checked::CheckedShr
            + wrapping::WrappingShl
            + wrapping::WrappingShl
            + Num
            + PrimInt
            + core::fmt::Debug,
        E: EnumMetadata<EnumT = E, Repr = R>,
        O: Copy + EnumMetadata<EnumT = E, Repr = R>,
    > EnumMaskIter for O
{
    type I = EnumMaskIterator<R, E, O>;

    fn mask_iter(&self) -> EnumMaskIterator<R, E, O> {
        let nextpos = |x: R| {
            let pos: usize = x.trailing_zeros() as usize;
            if pos >= E::EnumT::REPR_SIZE * 8_usize {
                None
            } else {
                let one_r: R = num_traits::identities::one();
                Some(one_r << pos)
            }
        };
        let mask = self.to_repr();

        EnumMaskIterator {
            mask,
            step: nextpos(mask),
            phantom: core::marker::PhantomData,
        }
    }
}

pub trait EnumMaskIter: Sized
where
    Self: EnumMetadata,
    Self::Repr: core::ops::BitOr + core::ops::BitOrAssign,
    <Self as EnumMetadata>::EnumT: EnumMetadata,
{
    type I: Iterator<Item = Self::EnumT>;

    fn mask_iter(&self) -> Self::I;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumMaskIterator<
    R: Copy
        + ops::BitOr
        + ops::BitAnd
        + ops::BitXor
        + ops::Shr
        + ops::Shl
        + ops::Not
        + ops::BitOrAssign
        + ops::BitAndAssign
        + ops::BitXorAssign
        + ops::ShrAssign
        + ops::ShlAssign
        + checked::CheckedShl
        + checked::CheckedShr
        + wrapping::WrappingShl
        + wrapping::WrappingShl
        + Num
        + PrimInt
        + core::fmt::Debug,
    E: EnumMetadata<Repr = R, EnumT = E>,
    O: EnumMetadata<Repr = R, EnumT = E>,
> {
    mask: R,
    step: Option<R>,
    phantom: core::marker::PhantomData<(O, R)>,
}

impl<
        R: Copy
            + ops::BitOr
            + ops::BitAnd
            + ops::BitXor
            + ops::Shr
            + ops::Shl
            + ops::Not
            + ops::BitOrAssign
            + ops::BitAndAssign
            + ops::BitXorAssign
            + ops::ShrAssign
            + ops::ShlAssign
            + checked::CheckedShl
            + checked::CheckedShr
            + wrapping::WrappingShl
            + wrapping::WrappingShl
            + Num
            + PrimInt
            + core::fmt::Debug,
        E: EnumMetadata<Repr = R, EnumT = E>,
        O: EnumMetadata<Repr = R, EnumT = E>,
    > Iterator for EnumMaskIterator<R, E, O>
{
    type Item = <E as EnumMetadata>::EnumT;

    fn next(&mut self) -> Option<Self::Item> {
        let nextpos = |x: R| {
            let pos: usize = x.trailing_zeros() as usize;
            if pos >= (E::EnumT::REPR_SIZE * 8_usize) as usize {
                None
            } else {
                let one_r: R = num_traits::identities::one();
                Some(one_r << pos)
            }
        };

        if let Some(step) = self.step {
            let mut ret = None;
            while let None = ret {
                let proposed_repr = step & self.mask;

                // Assumption: the single 1 bit in Some(step) is also 1 in self.mask.
                assert_eq!(proposed_repr, step);
                ret = E::EnumT::from_repr(proposed_repr);
                // Strip that bit out of mask.
                self.mask ^= step;
                self.step = nextpos(self.mask);
                if let None = self.step {
                    break;
                }
            }
            ret
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // TODO if we know which bits will never be in a valid repr,
        // and that the other single bits all represent valid representations
        // We could return an exact lower == upper bounds with `count_ones()`,
        // then derive ExactSizeIterator.
        (0, Some(self.mask.count_ones() as usize))
    }
}

#[test]
fn mask_iter() {
    let mask = ABC::A.opaque_repr();
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::A]);
    let mask = ABC::A.opaque_repr() | ABC::B;
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::A, ABC::B]);
    let mask = ABC::C.opaque_repr() | ABC::B.opaque_repr();
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::B, ABC::C]);
}

#[test]
fn check_opaque_zero_repr() {
    use strum::OpaqueRepr;
    let mask: OpaqueRepr<ABC> = OpaqueRepr::zero();
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), []);
}

#[derive(Debug, Eq, PartialEq, EnumMetadata)]
#[repr(u8)]
enum ReprBoundary {
    End = 1 << 7,
}

#[test]
fn test_repr_boundary() {
    let mask = ReprBoundary::End.opaque_repr();
    assert_eq!(
        mask.mask_iter().collect::<Vec<ReprBoundary>>(),
        [ReprBoundary::End]
    );
}

#[derive(Debug, Eq, PartialEq, EnumMetadata)]
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
    let mask = A.opaque_repr();
    assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [A]);
    let mask = A.opaque_repr() | H;
    assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [A, H]);
    let mask = B.opaque_repr() | H;
    assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [B, H]);
    let mask = B.opaque_repr() | G;
    assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [B, G]);
    let mask = G.opaque_repr() | B;
    assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [B, G]);
}
