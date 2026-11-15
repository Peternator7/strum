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
        #[strum(to_string = "gray {0}%")]
        Gray(u8),
    }

    #[test]
    fn from_str_no_std() {
        assert_eq!(Color::Yellow, Color::from_str("yellow").unwrap());
    }

    struct Buf {
        data: [u8; 16],
        len: usize,
    }

    impl core::fmt::Write for Buf {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let end = self.len + s.len();
            self.data
                .get_mut(self.len..end)
                .ok_or(core::fmt::Error)?
                .copy_from_slice(s.as_bytes());
            self.len = end;
            Ok(())
        }
    }

    #[test]
    fn display_interpolated_no_std() {
        use core::fmt::Write;
        let mut buf = Buf {
            data: [0; 16],
            len: 0,
        };
        write!(buf, "{:>10}", Color::Gray(50)).unwrap();
        assert_eq!(b"  gray 50%", &buf.data[..buf.len]);
    }

    #[test]
    fn try_from_str_no_std() {
        use core::convert::TryFrom;
        assert_eq!(Color::Yellow, Color::try_from("yellow").unwrap());
    }
}
