use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Debug, EnumCount, EnumIter)]
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
    assert_eq!(7, Week::COUNT);
    assert_eq!(Week::iter().count(), Week::COUNT);
}

#[test]
fn crate_module_path_test() {
    pub mod nested {
        pub mod module {
            pub use strum;
        }
    }

    #[derive(Debug, EnumCount, EnumIter)]
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

    assert_eq!(7, Week::COUNT);
    assert_eq!(Week::iter().count(), Week::COUNT);
}
