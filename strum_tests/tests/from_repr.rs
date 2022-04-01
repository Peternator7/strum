use core::convert::TryFrom;
use core::convert::TryInto;

use strum::FromRepr;

#[derive(Debug, FromRepr, PartialEq)]
#[repr(u8)]
enum Week {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday = 4 + 3,
    Saturday = 8,
}

macro_rules! assert_eq_repr {
    ( $type:ident::from_repr($number:literal), Some($enum:expr) ) => {
        assert_eq!($type::from_repr($number), Some($enum));
        assert_eq!(TryInto::<$type>::try_into($number), Ok($enum));
        assert_eq!(<$type as TryFrom<_>>::try_from($number), Ok($enum));
    };
    ( $type:ident::from_repr($number:literal), None ) => {
        assert_eq!($type::from_repr($number), None);
        assert_eq!(TryInto::<$type>::try_into($number), Err(::strum::ParseError::VariantNotFound));
        assert_eq!(<$type as TryFrom<_>>::try_from($number), Err(::strum::ParseError::VariantNotFound));
    };
}

#[test]
fn simple_test() {
    assert_eq_repr!(Week::from_repr(0), Some(Week::Sunday));
    assert_eq_repr!(Week::from_repr(1), Some(Week::Monday));
    assert_eq_repr!(Week::from_repr(6), None);
    assert_eq_repr!(Week::from_repr(7), Some(Week::Friday));
    assert_eq_repr!(Week::from_repr(8), Some(Week::Saturday));
    assert_eq_repr!(Week::from_repr(9), None);
}

#[rustversion::since(1.46)]
#[test]
fn const_test() {
    // This is to test that it works in a const fn
    const fn from_repr(discriminant: u8) -> Option<Week> {
        Week::from_repr(discriminant)
    }
    assert_eq!(from_repr(0), Some(Week::Sunday));
    assert_eq!(from_repr(1), Some(Week::Monday));
    assert_eq!(from_repr(6), None);
    assert_eq!(from_repr(7), Some(Week::Friday));
    assert_eq!(from_repr(8), Some(Week::Saturday));
    assert_eq!(from_repr(9), None);
}

#[test]
fn crate_module_path_test() {
    pub mod nested {
        pub mod module {
            pub use strum;
        }
    }

    #[derive(Debug, FromRepr, PartialEq)]
    #[strum(crate = "nested::module::strum")]
    enum Week {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }

    assert_eq_repr!(Week::from_repr(0), Some(Week::Sunday));
    assert_eq_repr!(Week::from_repr(6), Some(Week::Saturday));
    assert_eq_repr!(Week::from_repr(7), None);
}
