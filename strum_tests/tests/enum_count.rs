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

// EnumIter doesn't support lifetimes so we can't check consistency with that.
#[derive(Debug, EnumCount)]
enum HasLifetime<'a> {
    Hello(&'a str),
}

#[test]
fn lifetime_test() {
    let _ = HasLifetime::Hello("world");
    assert_eq!(1, HasLifetime::COUNT);
}
