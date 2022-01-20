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
    assert_eq!(ABC::from_repr(ABC::A as u8), <ABC as EnumMetadata>::from_repr(ABC::A.to_repr()))
}

// Scaffolding for further tests, and an example
// Iterator type which uses the EnumMetadata trait.
impl<
        R: num_traits::PrimInt
            + core::ops::BitOrAssign
            + num_traits::WrappingShr
            + num_traits::WrappingShl,
        E: EnumMetadata<EnumT = E, Repr = R>,
        O: Copy + EnumMetadata<EnumT = E, Repr = R>,
    > EnumMaskIter for O
{
    type I = EnumMaskIterator<R, E, O>;

    fn mask_iter(&self) -> EnumMaskIterator<R, E, O> {
        EnumMaskIterator {
            mask: self.to_repr(),
            shift: 0,
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
    R: num_traits::int::PrimInt
        + std::ops::BitOrAssign
        + num_traits::ops::wrapping::WrappingShl
        + num_traits::ops::wrapping::WrappingShr,
    E: EnumMetadata<Repr = R, EnumT = E>,
    O: EnumMetadata<Repr = R, EnumT = E>,
> {
    mask: R,
    shift: u32,
    phantom: core::marker::PhantomData<(O, R)>,
}

impl<
        R: num_traits::WrappingShr
            + num_traits::WrappingShl
            + num_traits::int::PrimInt
            + std::ops::BitOrAssign,
        E: EnumMetadata<Repr = R, EnumT = E>,
        O: EnumMetadata<Repr = R, EnumT = E>,
    > Iterator for EnumMaskIterator<R, E, O>
{
    type Item = <E as EnumMetadata>::EnumT;

    fn next(&mut self) -> Option<Self::Item> {
        // This can doubtlessly be improved
        if self.shift >= std::mem::size_of::<R>() as u32 * 8 {
            return None;
        }

        let tz: u32 = self.mask.trailing_zeros();
        let tz = if tz < std::mem::size_of::<R>() as u32 * 8 {
            tz
        } else {
            std::mem::size_of::<R>() as u32 * 8 - 1
        };
        let discr = ((self.mask.wrapping_shr(tz)) & num_traits::identities::one())
            .wrapping_shl(self.shift + tz);

        let one_u32: u32 = num_traits::identities::one();
        self.mask = self.mask.wrapping_shr(tz + one_u32);
        let shift_lhs: u32 = core::ops::Add::<u32>::add(tz, one_u32);
        self.shift += shift_lhs;
        E::EnumT::from_repr(discr)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // FIXME This assumes mask bits only contain enum repr items...
        // And there is only one bit to each enum discriminant.
        (
            self.mask.count_ones() as usize,
            Some(self.mask.count_ones() as usize),
        )
    }
}

// FIXME derive ExactSizeIterator...

#[test]
fn mask_iter() {
    let mask = ABC::A.opaque_repr();
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::A]);
    let mask = ABC::A.opaque_repr() | ABC::B;
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::A, ABC::B]);
    let mask = ABC::C.opaque_repr() | ABC::B.opaque_repr();
    assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::B, ABC::C]);
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
