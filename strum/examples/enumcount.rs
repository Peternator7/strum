use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, EnumCountMacro, EnumIter)]
enum Week {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn main() {
    assert_eq!(7, Week::COUNT);
    assert_eq!(Week::iter().count(), Week::COUNT);
}
