#[cfg(test)]
mod tests;

extern crate strenum_macro;

/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use strenum_macro::StrEnum;
///
/// #[derive(Debug, PartialEq, StrEnum)]
///     enum Letter<'a, T> {
///         #[strenum("-")]
///         A,
///         #[strenum("hello")]
///         B,
///         #[strenum(disabled)]
///         C(&'a str, T),
///         D
///     }
/// let a = Letter::<i32>::from_str("-").unwrap();
/// let b = Letter::<i32>::from_str("hello").unwrap();
/// assert_eq!(Letter::A, a);
/// assert_eq!(Letter::B, b);
/// ```
pub use strenum_macro::StrEnum;
