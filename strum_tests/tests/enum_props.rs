use strum::EnumProperty;

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
    use strum as custom_module_path;

    #[allow(dead_code)]
    #[derive(Debug, EnumProperty)]
    #[strum(Crate = "custom_module_path")]
    enum Test {
        #[strum(props(key = "value"))]
        A,
        B,
    }

    let a = Test::A;
    assert_eq!("value", a.get_str("key").unwrap());
}
