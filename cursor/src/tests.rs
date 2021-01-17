#[cfg(test)]
mod cursor_tests {
    use crate::Cursor;

    #[test]
    fn bump_test() {
        let mut cursor = Cursor::new("Alice Bob");
        cursor.bump_n(2);
        assert_eq!(cursor.eaten_len(), 3);
    }

    #[test]
    fn eat_id_test() {
        let mut cursor = Cursor::new("hello rust world+bye");
        assert_eq!(cursor.eat_id(), 5);
        assert_eq!(cursor.bump(), ' ');
        assert_eq!(cursor.eat_id(), 4);
        assert_eq!(cursor.bump(), ' ');
        assert_eq!(cursor.eat_id(), 5);
        assert_eq!(cursor.bump(), '+');
        assert_eq!(cursor.eat_id(), 3);
    }
}
