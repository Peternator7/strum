use strum::EnumTryAs;

#[derive(EnumTryAs)]
enum Foo {
    Unnamed0(),
    Unnamed1(u128),
    Unnamed2(bool, String),
    #[strum(disabled)]
    #[allow(dead_code)]
    Disabled(u32),
    #[allow(dead_code)]
    Unit,
    #[allow(dead_code)]
    Named { _a: u32, _b: String },
}

#[test]
fn unnamed_0() {
    let foo = Foo::Unnamed0();
    assert_eq!(Some(()), foo.try_as_unnamed_0());
}

#[test]
fn unnamed_1() {
    let foo = Foo::Unnamed1(128);
    assert_eq!(Some(&128), foo.try_as_unnamed_1_ref());
}

#[test]
fn unnamed_2() {
    let foo = Foo::Unnamed2(true, String::from("Hay"));
    assert_eq!(Some((true, String::from("Hay"))), foo.try_as_unnamed_2());
}

#[test]
fn can_mutate() {
    let mut foo = Foo::Unnamed1(128);
    if let Some(value) = foo.try_as_unnamed_1_mut() {
        *value = 44_u128;
    }
    assert_eq!(foo.try_as_unnamed_1(), Some(44));
}

#[test]
fn doesnt_match_other_variations() {
    let foo = Foo::Unnamed1(66);
    assert_eq!(None, foo.try_as_unnamed_0());
}
