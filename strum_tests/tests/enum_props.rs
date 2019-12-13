#![allow(unused_imports)]
extern crate strum;
#[macro_use]
extern crate strum_macros;

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
