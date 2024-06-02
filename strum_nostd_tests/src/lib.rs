#![no_std]

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use strum::EnumString;

    #[derive(Debug, Eq, PartialEq, EnumString, strum::Display)]
    enum Color {
        Red,
        Blue {
            hue: usize,
        },
        #[strum(serialize = "y", serialize = "yellow")]
        Yellow,
        #[strum(to_string = "purp")]
        Purple,
        #[strum(serialize = "blk", serialize = "Black", ascii_case_insensitive)]
        Black,
    }

    #[test]
    fn from_str_no_std() {
        assert_eq!(Color::Yellow, Color::from_str("yellow").unwrap());
    }

    #[test]
    #[rustversion::since(1.34)]
    fn try_from_str_no_std() {
        use core::convert::TryFrom;
        assert_eq!(Color::Yellow, Color::try_from("yellow").unwrap());
    }

    #[test]
    #[rustversion::before(1.34)]
    fn try_from_str_no_std() {}
}
