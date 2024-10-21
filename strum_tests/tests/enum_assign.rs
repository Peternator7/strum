use strum_macros::EnumAssign;
#[derive(EnumAssign)]
enum Foo {
    Unit(),
    Named0(),
    Named1 {
        _a: char,
    },
    Named2 {
        _a: u32,
        _b: String,
    },
    Unnamed0(),
    Unnamed1(Option<u128>),
    Unnamed2(bool, u8),
    MultiWordName(),
    #[strum(disabled)]
    #[allow(dead_code)]
    Disabled(bool),
}

#[test]
fn test_func() {
    let n1 = Foo::Named1 { _a: 'a' };

    if let Some(g) = n1.groups().g_char {
        println!("{}", g);
    }
}