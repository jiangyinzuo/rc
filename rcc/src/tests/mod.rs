use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

#[cfg(test)]
mod rcc_tests;

pub fn read_from_file(file_name: &str, path: &str) -> String {
    let mut file = File::open(format!("{}/{}", path, file_name)).unwrap();
    let mut expected = String::new();
    file.read_to_string(&mut expected).unwrap();
    expected
}

#[inline]
pub fn assert_pretty_fmt_eq<T: Debug + PartialEq>(expected: &str, actual: &T) {
    assert_eq!(expected, format!("{:#?}", actual));
}

#[inline]
pub fn assert_fmt_eq<T: Debug + PartialEq>(expected: &str, actual: &T) {
    assert_eq!(expected, format!("{:?}", actual));
}
