extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::IntoEnumIterator;

#[derive(Debug,Eq,PartialEq,EnumIter)]
enum Week {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[test]
fn simple_test() {
    let results = Week::iter().collect::<Vec<_>>();
    let expected = vec![Week::Sunday,
                        Week::Monday,
                        Week::Tuesday,
                        Week::Wednesday,
                        Week::Thursday,
                        Week::Friday,
                        Week::Saturday];

    assert_eq!(expected, results);
}

#[derive(Debug,Eq,PartialEq,EnumIter)]
enum Complicated<U: Default, V: Default> {
    A(U),
    B { v: V },
    C,
}

#[test]
fn complicated_test() {
    let results = Complicated::iter().collect::<Vec<_>>();
    let expected = vec![Complicated::A(0),
                        Complicated::B { v: String::new() },
                        Complicated::C];

    assert_eq!(expected, results);
}
