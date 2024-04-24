//! Anonymous struct implementation in rust.
//!
//! # Overview
//!
//! ## Import
//!
//! Basically, structz only has procedural macros, which references types from
//! [tuplez](https://docs.rs/tuplez) and [stringz](https://docs.rs/stringz),
//! so import them all:
//!
//! ```bash
//! cargo add structz tuplez stringz
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

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated};

mod parse;

use parse::*;

/// Create anonymous struct object.
///
/// Just like how an object of named struct is created, you declare a field name,
/// followed by a colon, and then its value. Finally, you separate each field with commas.
///
/// ```
/// use structz::*;
///
/// let son_age = 12;
/// let father = stru! {
///     name: "John",
///     age: son_age + 25,
/// };
/// assert_eq!(field!(father.age), 37);
/// ```
///
/// When field's value is omitted, it captures the value of the variable with the same name
/// in the context.
///
/// ```
/// use structz::*;
///
/// let name = "Smith";
/// let person = stru! {
///     name,
///     age: 30,
/// };
/// assert_eq!(field!(person.name), "Smith");
/// ```
#[proc_macro]
pub fn stru(input: TokenStream) -> TokenStream {
    let AnonymousStruct(input) = parse_macro_input!(input as AnonymousStruct);
    let fields: Vec<_> = input
        .into_iter()
        .map(|(ident, expr)| match expr {
            Some(expr) => quote! { (<::stringz::ident!(#ident)>::default(), #expr) },
            None => quote! { (<::stringz::ident!(#ident)>::default(), #ident) },
        })
        .collect();
    quote! {
        ::tuplez::tuple!(#(#fields),*)
    }
    .into()
}

/// Generate anonymous struct type.
///
/// Sometimes you may need to know the exact type of an anonymous struct object.
///
/// Just like how a named struct type is declared, you declare a field name,
/// followed by a colon, and then its type. Finally, you separate each field with commas.
///
/// ```
/// use structz::*;
///
/// type Person = stru_t! {
///     name: &'static str,
///     age: u8,
/// };
///
/// let person: Person = stru! {
///     age: 15,
///     name: "Alice",
/// };
/// ```
///
/// For cases where the anonymous structs are used as function arguments, it is recommended
/// that you use the [`macro@named_args`] instead.
#[proc_macro]
pub fn stru_t(input: TokenStream) -> TokenStream {
    let AnonymousStructType(input) = parse_macro_input!(input as AnonymousStructType);
    let fields: Vec<_> = input
        .into_iter()
        .map(|(ident, ty)| quote! { (::stringz::ident!(#ident), #ty) })
        .collect();
    quote! {
        ::tuplez::tuple_t!(#(#fields),*)
    }
    .into()
}

/// Access a field of an anonymous struct object.
///
/// ```
/// use structz::*;
///
/// let mut person = stru! {
///     name: "John Doe",
///     age: 26,
///     tags: vec!["developer", "rustacean"],
/// };
///
/// // immutable borrow
/// assert_eq!(field!(&person.name), &"John Doe");
///
/// // mutable borrow
/// *field!(&mut person.age) += 1;
/// assert_eq!(field!(&person.age), &27);
///
/// // consume the struct and get the field value
/// let tags = field!(person.tags);
/// assert_eq!(tags, vec!["developer", "rustacean"]);
///
/// // `person` cannot be used anymore.
/// ```
///
/// Unlike the built-in member access operator `.`, structz uses some type deduction magic
/// to achieve access to the specified field. Therefore, it is not smart enough:
/// it cannot move a value of certain field out while keeping other fields remain valid,
/// and it also cannot avoid consume the entire struct object while moving a value that
/// implements `Copy` out.
///
/// However, if all fields implement `Copy`, then the anonymous struct will also implement `Copy`:
///
/// ```
/// use structz::*;
///
/// let pos = stru! {
///     x: 300,
///     y: 480,
///     marker: "Block",
/// };
/// assert_eq!(field!(pos.x), 300);
/// assert_eq!(field!(pos.y), 480);
/// assert_eq!(field!(pos.marker), "Block");
/// ```
#[proc_macro]
pub fn field(input: TokenStream) -> TokenStream {
    let GetField(flag, value, field) = parse_macro_input!(input as GetField);
    match flag {
        RefFlag::None => quote! {
            {
                use ::tuplez::TupleLike;
                let ((_, field), _): ((::stringz::ident!(#field), _), _) = (#value).take();
                field
            }
        },
        RefFlag::Ref => quote! {
            {
                use ::tuplez::TupleLike;
                let (_, field): &(::stringz::ident!(#field), _) = (#value).get_ref();
                field
            }
        },
        RefFlag::Mut => quote! {
            {
                use ::tuplez::TupleLike;
                let (_, field): &mut (::stringz::ident!(#field), _) = (#value).get_mut();
                field
            }
        },
    }
    .into()
}

/// Obtain a new struct, each field is an immutable reference to the corresponding field of the input struct.
/// 
/// ```
/// use structz::*;
/// 
/// let person = stru! {
///     name: "John",
///     age: 30,
///     tags: vec!["smart", "handsome"],
/// };
/// let ref_person = as_ref!(person);
/// assert_eq!(field!(ref_person.age), &30);
/// ```
#[proc_macro]
pub fn as_ref(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as syn::Ident);
    quote! {
        {
            use ::tuplez::TupleLike;
            #ident.as_ref().foreach(::tuplez::mapper! {
                <'a, T: Copy, U> | x: &'a (T, U) | -> (T, &'a U) {
                    (x.0, &x.1)
                }
            })
        }
    }
    .into()
}

/// Obtain a new struct, each field is a mutable reference to the corresponding field of the input struct.
/// 
/// ```
/// use structz::*;
/// 
/// let mut person = stru! {
///     name: "John",
///     age: 30,
///     tags: vec!["smart", "handsome"],
/// };
/// let mut_person = as_mut!(person);
/// field!(mut_person.tags).pop();
/// assert_eq!(field!(&person.tags), &vec!["smart"]);
/// ```
#[proc_macro]
pub fn as_mut(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as syn::Ident);
    quote! {
        {
            use ::tuplez::TupleLike;
            #ident.as_mut().foreach(::tuplez::mapper! {
                <'a, T: Copy, U> | x: &'a mut (T, U) | -> (T, &'a mut U) {
                    (x.0, &mut x.1)
                }
            })
        }
    }
    .into()
}

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
#[proc_macro_attribute]
pub fn named_args(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemFn);
    let mut args = vec![];
    let mut has_self = None;
    for arg in input.sig.inputs {
        match arg {
            syn::FnArg::Receiver(arg) => has_self = Some(arg),
            syn::FnArg::Typed(arg) => {
                if let syn::Pat::Ident(pat) = *arg.pat {
                    args.push((pat, arg.ty))
                } else {
                    return quote! {
                       compile_error!("arguments must be an identifier or receiver");
                    }
                    .into();
                }
            }
        }
    }
    args.sort_by(|x, y| x.0.ident.cmp(&y.0.ident));
    let (idents, tys): (Vec<syn::PatIdent>, Vec<Box<syn::Type>>) = args.into_iter().unzip();
    let struct_type: syn::Type = parse_quote! {
        structz::stru_t! { #(#idents: #tys),* }
    };
    let mut inputs = Punctuated::new();
    if let Some(arg) = has_self {
        inputs.push(syn::FnArg::Receiver(arg));
    }
    inputs.push(syn::FnArg::Typed(syn::PatType {
        attrs: vec![],
        pat: parse_quote! { structz_s },
        colon_token: Default::default(),
        ty: Box::new(struct_type),
    }));
    input.sig.inputs = inputs;
    let mut unpack: Vec<(syn::Pat, syn::Type)> = vec![];
    for ident in idents {
        let ident = ident.ident;
        unpack.push((
            parse_quote! { #ident },
            parse_quote! { ::stringz::ident!(#ident) },
        ))
    }
    let (pat, ident_tys): (Vec<syn::Pat>, Vec<syn::Type>) = unpack.into_iter().unzip();
    let pat: syn::Pat = parse_quote! { ::tuplez::tuple_pat!( #( (_, #pat) ),* )};
    let ident_tys: syn::Type = parse_quote! { ::tuplez::tuple_t!( #( (#ident_tys, #tys) ),* )};
    let unpack: syn::Stmt = parse_quote! { let #pat: #ident_tys = structz_s; };
    input.block.stmts.insert(0, unpack);
    quote!(#input).into()
}

extern crate self as structz;
