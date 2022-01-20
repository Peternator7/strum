use strum::EnumRepr;
use strum_macros::EnumMask;

impl<
        R: num_traits::PrimInt
            + core::ops::BitOrAssign
            + num_traits::WrappingShr
            + num_traits::WrappingShl,
        E: EnumRepr<EnumT = E, Repr = R>,
        O: Copy + EnumRepr<EnumT = E, Repr = R>,
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
    Self: EnumRepr,
    Self::Repr: core::ops::BitOr + core::ops::BitOrAssign,
    <Self as EnumRepr>::EnumT: EnumRepr,
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
    E: EnumRepr<Repr = R, EnumT = E>,
    O: EnumRepr<Repr = R, EnumT = E>,
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
        E: EnumRepr<Repr = R, EnumT = E>,
        O: EnumRepr<Repr = R, EnumT = E>,
    > Iterator for EnumMaskIterator<R, E, O>
{
    type Item = <T as EnumRepr>::EnumT;

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
        T::EnumT::cvt_from_repr(discr)
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
