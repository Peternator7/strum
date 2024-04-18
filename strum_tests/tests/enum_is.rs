use std::borrow::Cow;
use strum::EnumIs;

mod core {} // ensure macros call `::core`
#[derive(EnumIs)]
enum LifeTimeTest<'a> {
    One(Cow<'a, str>),
    Two(&'a str),
}
#[derive(EnumIs)]
enum Foo {
    Unit,
    Named0 {},
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
    MultiWordName,
    #[strum(disabled)]
    #[allow(dead_code)]
    Disabled,
}
#[test]
fn generics_test() {
    let foouname = LifeTimeTest::One(Cow::Borrowed("Hello"));
    assert!(foouname.is_one());
    let foouname = LifeTimeTest::Two("Hello");
    assert!(foouname.is_two());
    let foouname = LifeTimeTest::One(Cow::Owned("Hello".to_string()));
    assert!(foouname.is_one());
}
#[test]
fn simple_test() {
    assert!(Foo::Unit.is_unit());
}

#[test]
fn named_0() {
    assert!(Foo::Named0 {}.is_named_0());
}

#[test]
fn named_1() {
    let foouname = Foo::Named1 {
        _a: Default::default(),
    };
    assert!(foouname.is_named_1());
}

#[test]
fn named_2() {
    let foouname = Foo::Named2 {
        _a: Default::default(),
        _b: Default::default(),
    };
    assert!(foouname.is_named_2());
}

#[test]
fn unnamed_0() {
    assert!(Foo::Unnamed0().is_unnamed_0());
}

#[test]
fn unnamed_1() {
    let foouname = Foo::Unnamed1(Default::default());
    assert!(foouname.is_unnamed_1());
}

#[test]
fn unnamed_2() {
    let foouname = Foo::Unnamed2(Default::default(), Default::default());
    assert!(foouname.is_unnamed_2());
}

#[test]
fn multi_word() {
    assert!(Foo::MultiWordName.is_multi_word_name());
}

#[test]
fn doesnt_match_other_variations() {
    assert!(!Foo::Unit.is_multi_word_name());
}
