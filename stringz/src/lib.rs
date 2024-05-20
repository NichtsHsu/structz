#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]

//! Convert strings to types to make it available as generic parameters.
//!
//! # Example
//!
//! ```
//! use stringz::{TypedString, string};
//!
//! fn test_hello<T: TypedString>() {
//!     assert_eq!(T::value(), "hello");
//! }
//!
//! test_hello::<string!("hello")>();
//! ```
//!
//! # Explanation
//!
//! The [`string`] macro converts `"hello"` to the following tuple type:
//!
//! ```text
//! (Character<'h'>, Character<'e'>, Character<'l'>, Character<'l'>, Character<'o'>)
//! ```
//!
//! Note: The above form is only for ease of understanding, the actual [`Tuple`] type of
//! [tuplez](https://docs.rs/tuplez) is used.
//!
//! All generated types are zero-sized types:
//!
//! ```
//! use stringz::string;
//! assert_eq!(std::mem::size_of::<string!("no matter how long it is")>(), 0);
//! ```

/// Convert a string to a type, the input must be a string literal.
///
/// # Example
///
/// ```
/// use stringz::{TypedString, string};
///
/// fn test_hello<T: TypedString>() {
///     assert_eq!(T::value(), "hello");
/// }
///
/// test_hello::<string!("hello")>();
/// ```
pub use stringz_macros::string;

/// Convert a string to a type, the input must be an identifier.
///
/// # Example
///
/// ```
/// use stringz::{TypedString, ident};
///
/// fn test_hello<T: TypedString>() {
///     assert_eq!(T::value(), "hello");
/// }
///
/// test_hello::<ident!(hello)>();
/// ```
pub use stringz_macros::ident;

use tuplez::{Tuple, Unit};

extern crate self as stringz;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{
    format,
    string::{String, ToString},
};

/// Single `char` type value representation.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Character<const C: char>;

/// Get original string from typed string (requires `alloc` or `std` feature).
pub trait TypedString {
    /// The original string.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
    fn value() -> String;
}

impl<const C: char> TypedString for Tuple<Character<C>, Unit> {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn value() -> String {
        C.to_string()
    }
}

impl<const C: char, Other> TypedString for Tuple<Character<C>, Other>
where
    Other: TypedString,
{
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn value() -> String {
        format!("{}{}", C, Other::value())
    }
}
