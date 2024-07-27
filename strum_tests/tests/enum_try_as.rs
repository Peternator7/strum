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
    Named {
        _a: u32,
        _b: String,
    },
}

#[test]
fn unnamed_0() {
    assert_eq!(Some(()), Foo::Unnamed0().try_as_unnamed_0());
}

#[test]
fn unnamed_1() {
    assert_eq!(Some(&128), Foo::Unnamed1(128).try_as_unnamed_1_ref());
}

#[test]
fn unnamed_2() {
    assert_eq!(
        Some((true, String::from("Hay"))),
        Foo::Unnamed2(true, String::from("Hay")).try_as_unnamed_2()
    );
}

#[test]
#[allow(clippy::disallowed_names)]
fn can_mutate() {
    let mut foo = Foo::Unnamed1(128);
    if let Some(value) = foo.try_as_unnamed_1_mut() {
        *value = 44_u128;
    }
    assert_eq!(foo.try_as_unnamed_1(), Some(44));
}

#[test]
fn doesnt_match_other_variations() {
    assert_eq!(None, Foo::Unnamed1(66).try_as_unnamed_0());
}
