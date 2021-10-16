use strum::{EnumConstIndex};

#[derive(Debug, EnumConstIndex, PartialEq)]
enum Week {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[test]
fn simple_test() {
    assert_eq!(Week::const_get(0), Some(Week::Sunday));
    assert_eq!(Week::const_get(6), Some(Week::Saturday));
    assert_eq!(Week::const_get(7), None);
}

#[test]
fn crate_module_path_test() {
    pub mod nested {
        pub mod module {
            pub use strum;
        }
    }

    #[derive(Debug, EnumConstIndex, PartialEq)]
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

    assert_eq!(Week::const_get(0), Some(Week::Sunday));
    assert_eq!(Week::const_get(6), Some(Week::Saturday));
    assert_eq!(Week::const_get(7), None);
}
