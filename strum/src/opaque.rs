pub struct OpaqueRepr<E: EnumMetadata>(E::Repr, core::marker::PhantomData<E>);

impl<E: EnumMetadata> Clone for OpaqueRepr<E> {
    fn clone(&self) -> Self {
        OpaqueRepr(self.0, core::marker::PhantomData)
    }
}

impl<E: EnumMetadata> Copy for OpaqueRepr<E> {}

pub trait EnumMetadata {
    /// The repr type.
    type Repr: Copy
        + core::ops::BitOr
        + core::ops::BitOrAssign
        + num_traits::ops::wrapping::WrappingShl
        + num_traits::ops::wrapping::WrappingShl
        + num_traits::int::PrimInt;
    /// The opaque representation type.
    type OpaqueRepr;
    /// The enum type.
    type EnumT: EnumMetadata;

    /// Variant names
    const VARIANTS: &'static [&'static str];
    /// Number of variants
    const COUNT: usize;

    /// convert to the enums #[repr(..)]
    /// equivalent to `self as ..`
    fn to_repr(self) -> Self::Repr;
    /// Converts to a OpaqueRepr<Self>
    fn opaque_repr(self) -> Self::OpaqueRepr;
    /// Non-const trait version of FromRepr
    fn from_repr(repr: Self::Repr) -> Option<Self::EnumT>;
}

impl<E: EnumMetadata> core::ops::BitOr<E> for OpaqueRepr<E>
where
    Self: EnumMetadata<Repr = <E as EnumMetadata>::Repr>,
    <E as EnumMetadata>::Repr: core::ops::BitOr<Output = <E as EnumMetadata>::Repr>,
{
    type Output = Self;
    fn bitor(self, other: E) -> OpaqueRepr<E> {
        Self::from_repr_unchecked(self.to_repr() | other.to_repr())
    }
}

impl<E: EnumMetadata> core::ops::BitOr<Self> for OpaqueRepr<E>
where
    Self: EnumMetadata<Repr = <E as EnumMetadata>::Repr>,
    <E as EnumMetadata>::Repr: core::ops::BitOr<Output = <E as EnumMetadata>::Repr>,
{
    type Output = Self;
    fn bitor(self, other: OpaqueRepr<E>) -> OpaqueRepr<E> {
        Self::from_repr_unchecked(self.to_repr() | other.to_repr())
    }
}

impl<E: EnumMetadata<EnumT = E>> EnumMetadata for OpaqueRepr<E> {
    type Repr = <E as EnumMetadata>::Repr;
    type OpaqueRepr = Self;
    type EnumT = E;
    const VARIANTS: &'static [&'static str] = Self::EnumT::VARIANTS;
    const COUNT: usize = Self::EnumT::COUNT;

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
        OpaqueRepr::<E>(e.to_repr(), core::marker::PhantomData)
    }

    fn from_repr_unchecked(repr: E::Repr) -> OpaqueRepr<E> {
        OpaqueRepr::<E>(repr, core::marker::PhantomData)
    }
}

