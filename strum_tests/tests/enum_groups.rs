use strum_macros::EnumGroups;
#[derive(EnumGroups)]
enum Foo {
    Unit(),
    Group1_1 {
        _a: u32,
        _b: String,
    },
    Group1_2 {
        _c: u32,
        _d: String,
    },
    Group2_1(Option<u128>, bool),
    Group2_2(Option<u128>, bool),
    Enabled(bool),
    Error(ErrorString),
    #[strum(disabled)]
    #[allow(dead_code)]
    Disabled(bool),
}

#[test]
fn test_func() {
    // if you are confident in the results you may call `.unwrap()`
    let e1 = Foo::Group2_1(Some(5), true);
    assert_eq!(
        (Some(5), true),
        e1.get_groups().g_option_u128__bool.unwrap()
    );

    // otherwise you may use a `if let Some(var) = ...`
    let e2 = Foo::Group1_1 {
        _a: 0,
        _b: "Hello".to_string(),
    };
    if let Some((u, s)) = e2.get_groups().g_u32_string {
        assert_eq!((0, "Hello".to_string()), (u, s))
    }

    // Disabled Units will not get added to a group.
    // and will not have a group generated for them unless
    // there is another variant with the same args.
    let e3 = Foo::Disabled(true);
    assert_eq!(true, e3.get_groups().g_bool.is_none());

    // Create different groups my using custom types.
    let e4 = Foo::Error("MyError".to_string());
    assert_eq!("MyError", e4.get_groups().g_errorstring.unwrap());
}

type ErrorString = String;


