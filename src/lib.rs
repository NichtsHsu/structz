#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![no_std]

//! Anonymous struct implementation in rust.
//!
//! # Overview
//!
//! ## Install
//!
//! ```bash
//! cargo add structz
//! ```
//!
//! ## Create & access
//!
//! ```
//! use structz::*;
//!
//! let age = 26;
//! let mut person = stru! {
//!     name: "John Doe",
//!     age,    // captures the value of the variable with the same name
//!     tags: vec!["developer", "rustacean"],
//! };
//!
//! // immutable borrow
//! assert_eq!(field!(&person.name), &"John Doe");
//!
//! // mutable borrow
//! *field!(&mut person.age) += 1;
//! assert_eq!(field!(&person.age), &27);
//!
//! // consume the struct and get the field value
//! let tags = field!(person.tags);
//! assert_eq!(tags, vec!["developer", "rustacean"]);
//!
//! // `person` cannot be used anymore.
//! ```
//!
//! **NOTE**: anonymous structs with the same fields but different field orders
//! are considered structs of the same type.
//!
//! ```
//! use structz::*;
//!
//! let rect1 = stru! { width: 1920, height: 1080 };
//! let rect2 = stru! { height: 1080, width: 1920 };
//! assert_eq!(rect1, rect2);
//! ```
//!
//! ## As argument
//!
//! ```
//! use structz::*;
//!
//! fn print_person(person: stru_t! {name: &str, age: u8, tags: Vec<&str> }) {
//!     println!(
//!         "{} is {} years old and has tags {:?}",
//!         field!(&person.name),
//!         field!(&person.age),
//!         field!(&person.tags)
//!     );
//! }
//!
//! let person = stru! {
//!     tags: vec!["programmer", "artist"],
//!     name: "Alice",
//!     age: 30,
//! };
//! print_person(person);
//! ```
//!
//! A better way is to use the [`macro@named_args`] macro which helps you unpack the struct:
//!
//! ```
//! use structz::*;
//!
//! #[named_args]
//! fn print_person(name: &str, age: u8, tags: Vec<&str>) {
//!     println!("{} is {} years old and has tags {:?}", name, age, tags);
//! }
//!
//! let person = stru! {
//!     tags: vec!["programmer", "artist"],
//!     name: "Alice",
//!     age: 30,
//! };
//! print_person(person);
//! ```
//!
//! With the [`subseq()`](https://docs.rs/tuplez/latest/tuplez/trait.TupleLike.html#method.subseq)
//! method by [tuplez](https://docs.rs/tuplez), you can get a sub-struct of the anonymous struct:
//!
//! ```
//! use structz::*;
//! use tuplez::TupleLike;
//!
//! #[named_args]
//! fn print_person(name: &str, age: u8) {
//!     println!("{} is {} years old", name, age);
//! }
//!
//! let alice = stru! {
//!     jobs: "programmer",
//!     name: "Alice",
//!     age: 30,
//!     children: vec!["Bob"],
//! };
//! print_person(alice.subseq());
//!
//! let bob = stru! {
//!     name: "Bob",
//!     parent: vec!["Alice", "John"],
//!     age: 7,
//!     grade: 1,
//! };
//! print_person(bob.subseq());
//!
//! let empty = stru! {
//!     name: "**Empty**",
//!     age: 0,
//! };
//! print_person(empty.subseq());   // Of course it is a sub-struct of itself
//! ```
//!
//! ## As generic type
//!
//! ```
//! use structz::*;
//!
//! // `R1` and `R2` are "magic", used to indicate the position of the field in the structs,
//! // and these magic generic types will be automatically inferred by Rust.
//! // You should introduce a magic generic type for each field.
//! fn print_name_id<T, R1, R2>(any: &T)
//! where
//!     T: HasField<ident!(name), &'static str, R1>,
//!     T: HasField<ident!(id), usize, R2>,
//! {
//!     println!("{}", field!(&any.name));
//!     println!("{}", field!(&any.id));
//! }
//!
//! let person = stru! {
//!     name: "John",
//!     age: 15,
//!     id: 1006,
//!     jobs: "Programmer",
//! };
//! let earth = stru! {
//!     name: "Earth",
//!     id: 3,
//!     galaxy: "Sol",
//!     satellites: vec!["Moon"],
//! };
//! print_name_id(&person);
//! print_name_id(&earth);
//! ```
//!
//! # Details
//!
//! The implementation of structz is based on [stringz](https://docs.rs/stringz) and [tuplez](https://docs.rs/tuplez).
//!
//! First, macros sort the input fields in lexicographic order, which ensures that anonymous structs
//! with the same fields but different field orders are of the same type.
//!
//! Second, convert the field names into a specialized type consisting of a sequence of zero-sized types via
//! [stringz](https://docs.rs/stringz). Let's call them "field name types".
//!
//! Finally, pack the field name type and the data type of each field, combine them into
//! [tuplez](https://docs.rs/tuplez)'s [`Tuple`](https://docs.rs/tuplez/latest/tuplez/struct.Tuple.html).
//!
//! Since the field names is effectively replaced with a zero-sized type, it cost you nothing:
//!
//! ```
//! use structz::*;
//!
//! assert_eq!(
//!     std::mem::size_of::<
//!         stru_t! {
//!             age: u8,
//!             name: &'static str,
//!             tags: Vec<&'static str>,
//!         },
//!     >(),
//!     std::mem::size_of::<(u8, &'static str, Vec<&'static str>)>()
//! );
//! ```

#[macro_use]
mod macros;
mod has_field;

pub use has_field::*;

extern crate self as structz;

#[doc(hidden)]
pub use stringz as __stringz;
#[doc(hidden)]
pub use stringz::__tuplez;

#[doc(no_inline)]
pub use stringz::{ident, TypedString};

#[doc(hidden)]
pub use structz_macros::{stru as stru_inner, stru_t as stru_t_inner};

/// Change the function's arguments to an anonymous struct object and unpack it.
///
/// ```
/// use structz::*;
///
/// #[named_args]
/// fn print_person(name: &str, age: u8, tags: Vec<&str>) {
///     println!("{} is {} years old and has tags {:?}", name, age, tags);
/// }
///
/// print_person(stru! {
///     tags: vec!["programmer", "artist"],
///     name: "Alice",
///     age: 30,
/// });
/// ```
///
/// The method receivers are not considered part of the anonymous struct object:
///
/// ```
/// use structz::*;
///
/// struct Num(i32);
/// impl Num {
///     #[named_args]
///     fn add(&mut self, x: i32, y: i32) {
///         self.0 += x;
///         self.0 += y;
///     }
/// }
///
/// let mut num = Num(1);
/// num.add(stru! { x: 2, y: 3 });
/// assert_eq!(num.0, 6);
/// ```
pub use structz_macros::named_args;
