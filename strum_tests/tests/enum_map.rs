use strum::EnumMap;

#[derive(EnumMap)]
enum Color {
    Red,
    Yellow,
    Green,
    #[strum(disabled)]
    Teal,
    Blue,
    #[strum(disabled)]
    Indigo,
}

#[derive(EnumMap)]
enum Keyword {
    Const,
}

#[test]
fn default() {
    assert_eq!(ColorMap::default(), ColorMap::new(0, 0, 0, 0));
}

#[test]
#[should_panic]
fn disabled() {
    let _ = ColorMap::<u8>::default()[Color::Indigo];
}

#[test]
fn filled() {
    assert_eq!(ColorMap::filled(42), ColorMap::new(42, 42, 42, 42));
}

#[test]
fn from_closure() {
    assert_eq!(
        ColorMap::from_closure(|color| match color {
            Color::Red => 1,
            _ => 2,
        }),
        ColorMap::new(1, 2, 2, 2)
    );
}

#[test]
fn clone() {
    let cm = ColorMap::filled(String::from("Some Text Data"));
    assert_eq!(cm.clone(), cm);
}

#[test]
fn index() {
    let map = ColorMap::new(18, 25, 7, 2);
    assert_eq!(map[Color::Red], 18);
    assert_eq!(map[Color::Yellow], 25);
    assert_eq!(map[Color::Green], 7);
    assert_eq!(map[Color::Blue], 2);
}

#[test]
fn index_mut() {
    let mut map = ColorMap::new(18, 25, 7, 2);
    map[Color::Green] = 5;
    map[Color::Red] *= 4;
    assert_eq!(map[Color::Green], 5);
    assert_eq!(map[Color::Red], 72);
}

#[test]
fn option_all() {
    let mut map: ColorMap<Option<u8>> = ColorMap::filled(None);
    map[Color::Red] = Some(64);
    map[Color::Green] = Some(32);
    map[Color::Blue] = Some(16);

    assert_eq!(map.clone().all(), None);

    map[Color::Yellow] = Some(8);
    assert_eq!(map.all(), Some(ColorMap::new(64, 8, 32, 16)));
}

#[test]
fn result_all_ok() {
    let mut map: ColorMap<Result<u8, u8>> = ColorMap::filled(Ok(4));
    assert_eq!(map.clone().all_ok(), Ok(ColorMap::filled(4)));
    map[Color::Red] = Err(22);
    map[Color::Yellow] = Err(100);
    assert_eq!(map.clone().all_ok(), Err(22));
    map[Color::Red] = Ok(1);
    assert_eq!(map.all_ok(), Err(100));
}

#[test]
fn transform() {
    let all_two = ColorMap::filled(2);
    assert_eq!(all_two.transform(|_, n| *n * 2), ColorMap::filled(4));
}
