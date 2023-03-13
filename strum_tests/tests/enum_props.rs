use strum::EnumProperty;

mod core {} // ensure macros call `::core`

#[derive(Debug, EnumProperty)]
enum Test {
    #[strum(props(key = "value"))]
    A,
    B,
}

#[test]
fn prop_test() {
    let a = Test::A;
    assert_eq!("value", a.get_str("key").unwrap());
}

#[test]
fn prop_test_not_found() {
    let a = Test::A;
    assert_eq!(None, a.get_str("Not Found"));
}

#[test]
fn prop_test_not_found_2() {
    let b = Test::B;
    assert_eq!(None, b.get_str("key"));
}

#[test]
fn crate_module_path_test() {
    pub mod nested {
        pub mod module {
            pub use strum;
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, EnumProperty)]
    #[strum(crate = "nested::module::strum")]
    enum Test {
        #[strum(props(key = "value"))]
        A,
        #[strum(props(answer = 42))]
        B,
        #[strum(props(to_be = false))]
        C,
    }

    let a = Test::A;
    assert_eq!("value", a.get_str("key").unwrap());
    let b = Test::B;
    assert_eq!(42, b.get_int("answer").unwrap());
    let c = Test::C;
    assert_eq!(false, c.get_bool("to_be").unwrap());
}
