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
#[macro_export]
macro_rules! field {
    ($s:ident . $f:ident) => {
        ::structz::HasField::<::stringz::ident!($f), _, _>::take_field($s)
    };
    (& $s:ident . $f:ident) => {
        ::structz::HasField::<::stringz::ident!($f), _, _>::get_field(&$s)
    };
    (&* $s:ident . $f:ident) => {
        ::structz::HasField::<::stringz::ident!($f), _, _>::get_field(&*$s)
    };
    (&** $s:ident . $f:ident) => {
        ::structz::HasField::<::stringz::ident!($f), _, _>::get_field(&**$s)
    };
    (&mut $s:ident . $f:ident) => {
        ::structz::HasField::<::stringz::ident!($f), _, _>::get_field_mut(&mut $s)
    };
    (&mut * $s:ident . $f:ident) => {
        ::structz::HasField::<::stringz::ident!($f), _, _>::get_field_mut(&mut * $s)
    };
    (&mut ** $s:ident . $f:ident) => {
        ::structz::HasField::<::stringz::ident!($f), _, _>::get_field_mut(&mut ** $s)
    };
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
#[macro_export]
macro_rules! as_ref {
    ($e:expr) => {{
        use ::tuplez::TupleLike;
        $e.as_ref().foreach(::tuplez::mapper! {
            <'a, T: Copy, U> | x: &'a (T, U) | -> (T, &'a U) {
                (x.0, &x.1)
            }
        })
    }};
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
#[macro_export]
macro_rules! as_mut {
    ($e:expr) => {{
        use ::tuplez::TupleLike;
        $e.as_mut().foreach(::tuplez::mapper! {
            <'a, T: Copy, U> | x: &'a mut (T, U) | -> (T, &'a mut U) {
                (x.0, &mut x.1)
            }
        })
    }};
}
