use strum::EnumTryAs;

/// Tests that generated methods can be used in fully documented public APIs.
#[forbid(missing_docs)]
pub mod documented {
    /// A value with an optional payload.
    #[derive(strum::EnumTryAs)]
    pub enum Value {
        /// A numeric payload.
        Number(u32),
        /// No payload.
        Empty,
    }
}

#[test]
fn documented_public_enum() {
    let mut value = documented::Value::Number(1);
    assert_eq!(value.try_as_number_ref(), Some(&1));
    *value.try_as_number_mut().unwrap() = 2;
    assert_eq!(value.try_as_number(), Some(2));

    let mut empty = documented::Value::Empty;
    assert_eq!(empty.try_as_number_ref(), None);
    assert_eq!(empty.try_as_number_mut(), None);
    assert_eq!(empty.try_as_number(), None);
}

#[derive(EnumTryAs)]
enum Foo {
    Unnamed0(),
    Unnamed1(u128),
    Unnamed2(bool, String),
    Unnamed3(#[allow(deprecated)] u128),
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
fn unnamed_3() {
    let foo = Foo::Unnamed3(128);
    assert_eq!(Some(&128), foo.try_as_unnamed_3_ref());
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
