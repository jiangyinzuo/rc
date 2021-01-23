#[cfg(test)]
mod tests {
    extern crate strenum_macro;
    use std::str::FromStr;
    use strenum_macro::StrEnum;

    #[derive(StrEnum, PartialEq, Debug)]
    enum Color {
        Red,
        Green,
    }

    #[test]
    fn derive_test() {
        assert_eq!("red", Color::Red.to_string());
        assert_eq!(Color::Green, Color::from_str("green").unwrap());
    }
}
