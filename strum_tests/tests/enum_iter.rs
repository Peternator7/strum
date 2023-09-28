use strum::{EnumIter, IntoEnumIterator};

mod core {} // ensure macros call `::core`

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
    i.next();

    assert_eq!(0, i.size_hint().1.unwrap());
}

#[test]
fn double_ended_len_test() {
    let mut i = Complicated::<(), ()>::iter();
    assert_eq!(3, i.len());
    i.next_back();

    assert_eq!(2, i.len());
    i.next();

    assert_eq!(1, i.len());
    i.next_back();

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

#[test]
fn reverse_test() {
    let results = Week::iter().rev().collect::<Vec<_>>();
    let expected = vec![
        Week::Saturday,
        Week::Friday,
        Week::Thursday,
        Week::Wednesday,
        Week::Tuesday,
        Week::Monday,
        Week::Sunday,
    ];
    assert_eq!(expected, results);
}

#[test]
fn take_from_both_sides_test() {
    let mut iter = Week::iter();

    assert_eq!(Some(Week::Sunday), iter.next());
    assert_eq!(Some(Week::Saturday), iter.next_back());
    assert_eq!(Some(Week::Friday), iter.next_back());
    assert_eq!(Some(Week::Monday), iter.next());
    assert_eq!(Some(Week::Tuesday), iter.next());
    assert_eq!(Some(Week::Wednesday), iter.next());
    assert_eq!(Some(Week::Thursday), iter.next_back());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next_back());
}

#[test]
fn take_from_both_sides_test2() {
    let mut iter = Week::iter();

    assert_eq!(Some(Week::Sunday), iter.next());
    assert_eq!(Some(Week::Saturday), iter.next_back());
    assert_eq!(Some(Week::Friday), iter.next_back());
    assert_eq!(Some(Week::Monday), iter.next());
    assert_eq!(Some(Week::Tuesday), iter.next());
    assert_eq!(Some(Week::Wednesday), iter.next());
    assert_eq!(Some(Week::Thursday), iter.next());
    assert_eq!(None, iter.next_back());
    assert_eq!(None, iter.next());
}

#[test]
fn take_nth_test() {
    let mut iter = Week::iter();
    let saturday = iter.next_back();
    let friday = iter.next_back();
    let thursday = iter.next_back();

    assert_eq!(Some(Week::Tuesday), iter.nth(2));
    assert_eq!(Some(Week::Saturday), saturday);
    assert_eq!(Some(Week::Friday), friday);
    assert_eq!(Some(Week::Thursday), thursday);
    assert_eq!(None, iter.nth(1));
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next_back());
}

#[test]
fn crate_module_path_test() {
    pub mod nested {
        pub mod module {
            pub use strum;
        }
    }

    #[derive(Debug, Eq, PartialEq, EnumIter)]
    #[strum(crate = "nested::module::strum")]
    enum Week {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }

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

#[test]
fn enum_iter_option() {
    #[derive(Debug, Eq, PartialEq, EnumIter)]
    enum Option {
        BluePill,
        RedPill,
    }
    let results = Option::iter().collect::<Vec<_>>();
    let expected = vec![Option::BluePill, Option::RedPill];

    assert_eq!(expected, results);
}
