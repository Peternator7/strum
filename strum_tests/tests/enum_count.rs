extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::{EnumCount, IntoEnumIterator};

#[derive(Debug, EnumCount, EnumIter)]
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
    assert_eq!(7, Week::count());
    assert_eq!(Week::count(), WEEK_COUNT);
    assert_eq!(Week::iter().count(), WEEK_COUNT);
}
