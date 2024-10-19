use strum_macros::EnumAssign;
use crate::FooStructs::*;

mod core {} // ensure macros call `::core`
#[derive(EnumAssign)]
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
fn test_func() {

    let n1 = FooStruct::Named1(FOO_NAMED1_STRUCT { _a: '2' });
    let u2 = FooStruct::Unnamed2(FOO_UNNAMED2_STRUCT { _0: false, _1: 5 });

    println!("n1: {:?}", n1);
    tes(n1);
}

fn tes(t: FooStruct) -> FooStructsAll {
    match t {
        FooStruct::Named1(t) => {
            FooStructsAll {
                foo_named1_struct: None,
                foo_named2_struct: None,
                foo_unnamed1_struct: None,
                foo_unnamed2_struct: None,
            }
        },
        _ => {}
    }
}

