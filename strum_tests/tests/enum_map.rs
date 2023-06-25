use strum::EnumMap;

#[derive(EnumMap)]
enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}

#[test]
fn default() {
    assert_eq!(ColorMap::default(), ColorMap::new(0, 0, 0, 0));
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
fn transform() {
    let all_two = ColorMap::filled(2);
    assert_eq!(all_two.transform(|_, n| *n * 2), ColorMap::filled(4));
}
