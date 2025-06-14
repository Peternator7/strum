use strum::FromRepr;

mod core {} // ensure macros call `::core`

#[derive(Debug, FromRepr, PartialEq)]
/// Day of the week
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

#[test]
fn simple_test() {
    assert_eq!(Week::from_repr(0), Some(Week::Sunday));
    assert_eq!(Week::from_repr(1), Some(Week::Monday));
    assert_eq!(Week::from_repr(6), None);
    assert_eq!(Week::from_repr(7), Some(Week::Friday));
    assert_eq!(Week::from_repr(8), Some(Week::Saturday));
    assert_eq!(Week::from_repr(9), None);
}

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
            #[allow(unused_imports)]
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

    assert_eq!(Week::from_repr(0), Some(Week::Sunday));
    assert_eq!(Week::from_repr(6), Some(Week::Saturday));
    assert_eq!(Week::from_repr(7), None);
}
