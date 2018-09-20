extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq, EnumIter)]
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
    let expected = vec![
        Week::Sunday,
        Week::Monday,
        Week::Tuesday,
        Week::Wednesday,
        Week::Thursday,
        Week::Friday,
        Week::Saturday,
    ];

    assert_eq!(expected, results);
}

#[derive(Debug, Eq, PartialEq, EnumIter)]
enum Complicated<U: Default, V: Default> {
    A(U),
    B { v: V },
    C,
}

#[test]
fn complicated_test() {
    let results = Complicated::iter().collect::<Vec<_>>();
    let expected = vec![
        Complicated::A(0),
        Complicated::B { v: String::new() },
        Complicated::C,
    ];

    assert_eq!(expected, results);
}

#[test]
fn len_test() {
    let mut i = Complicated::<(), ()>::iter();
    assert_eq!(3, i.len());
    i.next();

    assert_eq!(2, i.len());
    i.next();

    assert_eq!(1, i.len());
    i.next();

    assert_eq!(0, i.len());
}

#[test]
fn clone_test() {
    let mut i = Week::iter();
    i.next();
    i.next();

    let mut i_cloned = i.clone();

    assert_eq!(Some(Week::Tuesday), i.next());
    assert_eq!(Some(Week::Tuesday), i_cloned.next());

    i.next();
    i.next();

    assert_eq!(Some(Week::Friday), i.next());
    assert_eq!(Some(Week::Wednesday), i_cloned.next());
}

#[test]
fn cycle_test() {
    let results = Week::iter().cycle().take(10).collect::<Vec<_>>();
    let expected = vec![
        Week::Sunday,
        Week::Monday,
        Week::Tuesday,
        Week::Wednesday,
        Week::Thursday,
        Week::Friday,
        Week::Saturday,
        Week::Sunday,
        Week::Monday,
        Week::Tuesday,
    ];
    assert_eq!(expected, results);
}
