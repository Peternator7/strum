use strum::{EnumCount, EnumIter, IntoEnumIterator};

mod core {} // ensure macros call `::core`

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

#[allow(dead_code)]
#[derive(Debug, EnumCount, EnumIter)]
enum Pets {
    Dog,
    Cat,
    Fish,
    Bird,
    #[strum(disabled)]
    Hamster,
}

#[test]
fn simple_test() {
    assert_eq!(7, Week::COUNT);
    assert_eq!(Week::iter().count(), Week::COUNT);
}

#[test]
fn disabled_test() {
    assert_eq!(4, Pets::COUNT);
    assert_eq!(Pets::iter().count(), Pets::COUNT);
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
