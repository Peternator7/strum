use strum::EnumIndex;

#[derive(Debug, EnumIndex, PartialEq)]
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
    assert_eq!(Week::index(0), Some(Week::Sunday));
    assert_eq!(Week::index(1), Some(Week::Monday));
    assert_eq!(Week::index(6), None);
    assert_eq!(Week::index(7), Some(Week::Friday));
    assert_eq!(Week::index(8), Some(Week::Saturday));
    assert_eq!(Week::index(9), None);
}

#[rustversion::since(1.46)]
#[test]
fn const_test() {
    assert_eq!(Week::const_index(0), Some(Week::Sunday));
    assert_eq!(Week::const_index(1), Some(Week::Monday));
    assert_eq!(Week::const_index(6), None);
    assert_eq!(Week::const_index(7), Some(Week::Friday));
    assert_eq!(Week::const_index(8), Some(Week::Saturday));
    assert_eq!(Week::const_index(9), None);
}

#[test]
fn crate_module_path_test() {
    pub mod nested {
        pub mod module {
            pub use strum;
        }
    }

    #[derive(Debug, EnumIndex, PartialEq)]
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

    assert_eq!(Week::index(0), Some(Week::Sunday));
    assert_eq!(Week::index(6), Some(Week::Saturday));
    assert_eq!(Week::index(7), None);
}
