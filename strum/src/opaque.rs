use core::{marker::PhantomData, ops};
use num_traits::int::PrimInt;
use num_traits::ops::{checked, wrapping};
use num_traits::Num;

use ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, Mul, Not, Rem, Shl,
    ShlAssign, Shr, ShrAssign, Sub,
};

pub struct OpaqueRepr<E: EnumMetadata>(E::Repr, PhantomData<E>);

impl<E: EnumMetadata> Clone for OpaqueRepr<E> {
    fn clone(&self) -> Self {
        OpaqueRepr(self.0, PhantomData)
    }
}

impl<E: EnumMetadata> Copy for OpaqueRepr<E> {}

pub trait EnumMetadata {
    /// The repr type.
    type Repr: Copy
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
        + core::fmt::Debug;

    /// The opaque representation type.
    type OpaqueRepr;
    /// The enum type.
    type EnumT: EnumMetadata;

    /// Variant names
    const VARIANTS: &'static [&'static str];
    /// Number of variants
    const COUNT: usize;
    /// std::mem::size_of<Self::R>().
    const REPR_SIZE: usize;

    /// convert to the enums #[repr(..)]
    /// equivalent to `self as ..`
    fn to_repr(self) -> Self::Repr;
    /// Converts to a OpaqueRepr<Self>
    fn opaque_repr(self) -> Self::OpaqueRepr;
    /// Non-const trait version of FromRepr
    fn from_repr(repr: Self::Repr) -> Option<Self::EnumT>;
}

macro_rules! binary_op {
    ($trait:ident, $op:ident) => {
        impl<E: EnumMetadata> $trait<E> for OpaqueRepr<E>
        where
            Self: EnumMetadata<Repr = <E as EnumMetadata>::Repr>,
            <E as EnumMetadata>::Repr: $trait<Output = <E as EnumMetadata>::Repr>,
        {
            type Output = Self;
            fn $op(self, other: E) -> OpaqueRepr<E> {
                Self::from_repr_unchecked(<Self as EnumMetadata>::Repr::$op(
                    self.to_repr(),
                    other.to_repr(),
                ))
            }
        }

        impl<E: EnumMetadata> $trait<Self> for OpaqueRepr<E>
        where
            Self: EnumMetadata<Repr = <E as EnumMetadata>::Repr>,
            <E as EnumMetadata>::Repr: $trait<Output = <E as EnumMetadata>::Repr>,
        {
            type Output = Self;
            fn $op(self, other: OpaqueRepr<E>) -> OpaqueRepr<E> {
                Self::from_repr_unchecked(<Self as EnumMetadata>::Repr::$op(
                    self.to_repr(),
                    other.to_repr(),
                ))
            }
        }
    };
}

macro_rules! unary_op {
    ($trait:ident, $f:ident) => {
        impl<E: EnumMetadata> $trait for OpaqueRepr<E>
        where
            Self: EnumMetadata<Repr = <E as EnumMetadata>::Repr>,
            <E as EnumMetadata>::Repr: $trait<Output = <E as EnumMetadata>::Repr>,
        {
            type Output = Self;
            fn $f(self) -> OpaqueRepr<E> {
                Self::from_repr_unchecked(<Self as EnumMetadata>::Repr::$f(self.to_repr()))
            }
        }
    };
}

macro_rules! binary_op_mut {
    ($trait:ident, $op:ident) => {
        impl<E: EnumMetadata> $trait<E> for OpaqueRepr<E>
        where
            Self: EnumMetadata<Repr = <E as EnumMetadata>::Repr>,
            <E as EnumMetadata>::Repr: $trait,
        {
            fn $op(&mut self, other: E) {
                <Self as EnumMetadata>::Repr::$op(&mut self.to_repr(), other.to_repr());
            }
        }

        impl<E: EnumMetadata> $trait<Self> for OpaqueRepr<E>
        where
            Self: EnumMetadata<Repr = <E as EnumMetadata>::Repr>,
            <E as EnumMetadata>::Repr: $trait,
        {
            fn $op(&mut self, other: OpaqueRepr<E>) {
                <Self as EnumMetadata>::Repr::$op(&mut self.to_repr(), other.to_repr());
            }
        }
    };
}

binary_op!(BitOr, bitor);
binary_op!(BitAnd, bitand);
binary_op!(BitXor, bitxor);
binary_op!(Shr, shr);
binary_op!(Shl, shl);
unary_op!(Not, not);

binary_op!(Add, add);
binary_op!(Sub, sub);
binary_op!(Mul, mul);
binary_op!(Div, div);
binary_op!(Rem, rem);

binary_op_mut!(BitOrAssign, bitor_assign);
binary_op_mut!(BitAndAssign, bitand_assign);
binary_op_mut!(BitXorAssign, bitxor_assign);
binary_op_mut!(ShrAssign, shr_assign);
binary_op_mut!(ShlAssign, shl_assign);

impl<E: EnumMetadata<EnumT = E>> EnumMetadata for OpaqueRepr<E> {
    type Repr = <E as EnumMetadata>::Repr;
    type OpaqueRepr = Self;
    type EnumT = E;

    const VARIANTS: &'static [&'static str] = Self::EnumT::VARIANTS;
    const COUNT: usize = Self::EnumT::COUNT;
    const REPR_SIZE: usize = Self::EnumT::REPR_SIZE;

    fn to_repr(self) -> Self::Repr {
        self.0
    }

    fn opaque_repr(self) -> Self {
        self
    }

    fn from_repr(repr: Self::Repr) -> Option<Self::EnumT> {
        Self::EnumT::from_repr(repr)
    }
}

impl<E: EnumMetadata> OpaqueRepr<E> {
    pub fn new(e: E) -> OpaqueRepr<E> {
        OpaqueRepr::<E>(e.to_repr(), PhantomData)
    }

    pub fn zero() -> OpaqueRepr<E> {
        OpaqueRepr::<E>(num_traits::identities::zero(), PhantomData)
    }

    fn from_repr_unchecked(repr: E::Repr) -> OpaqueRepr<E> {
        OpaqueRepr::<E>(repr, PhantomData)
    }
}
