extern crate strenum_macro;

/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use strenum_macro::EnumFromStr;
///
/// #[derive(Debug, PartialEq, EnumFromStr)]
///     enum Letter<'a, T> {
///         #[value("-")]
///         A,
///         #[value("hello", "WORLD")]
///         B,
///         #[disabled]
///         C(&'a str, T),
///         D
///     }
/// let a = Letter::<i32>::from_str("-").unwrap();
/// let b1 = Letter::<i32>::from_str("hello").unwrap();
/// let b2 = Letter::<i32>::from_str("WORLD").unwrap();
/// assert_eq!(Letter::A, a);
/// assert_eq!(Letter::B, b1);
/// assert_eq!(Letter::B, b2);
/// ```
pub use strenum_macro::EnumFromStr;
