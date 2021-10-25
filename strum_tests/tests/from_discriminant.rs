use strum::FromDiscriminant;

#[derive(Debug, FromDiscriminant, PartialEq)]
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
    assert_eq!(Week::from_discriminant(0), Some(Week::Sunday));
    assert_eq!(Week::from_discriminant(1), Some(Week::Monday));
    assert_eq!(Week::from_discriminant(6), None);
    assert_eq!(Week::from_discriminant(7), Some(Week::Friday));
    assert_eq!(Week::from_discriminant(8), Some(Week::Saturday));
    assert_eq!(Week::from_discriminant(9), None);
}

#[rustversion::since(1.46)]
#[test]
fn const_test() {
    // This is to test that it works in a const fn
    const fn from_discriminant(discriminant: u8) -> Option<Week> {
        Week::from_discriminant(discriminant)
    }
    assert_eq!(from_discriminant(0), Some(Week::Sunday));
    assert_eq!(from_discriminant(1), Some(Week::Monday));
    assert_eq!(from_discriminant(6), None);
    assert_eq!(from_discriminant(7), Some(Week::Friday));
    assert_eq!(from_discriminant(8), Some(Week::Saturday));
    assert_eq!(from_discriminant(9), None);
}

#[test]
fn crate_module_path_test() {
    pub mod nested {
        pub mod module {
            pub use strum;
        }
    }

    #[derive(Debug, FromDiscriminant, PartialEq)]
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

    assert_eq!(Week::from_discriminant(0), Some(Week::Sunday));
    assert_eq!(Week::from_discriminant(6), Some(Week::Saturday));
    assert_eq!(Week::from_discriminant(7), None);
}
