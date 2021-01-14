extern crate strenum_macro;

pub use strenum_macro::EnumFromStr;

mod tests {
    use std::str::FromStr;
    use strenum_macro::EnumFromStr;

    #[derive(Debug, PartialEq, EnumFromStr)]
    enum Letter {
        #[value("-")]
        A,
        #[value("hello", "WORLD")]
        B,
        C,
    }

    #[test]
    fn test() {
        let a = Letter::from_str("-").unwrap();
        let b1 = Letter::from_str("hello").unwrap();
        let b2 = Letter::from_str("WORLD").unwrap();
        assert_eq!(Letter::A, a);
        assert_eq!(Letter::B, b1);
        assert_eq!(Letter::B, b2);
    }
}
