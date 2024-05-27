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
        $crate::HasField::<$crate::ident!($f), _, _>::take_field($s)
    };
    (& $s:ident . $f:ident) => {{
        use $crate::__GetFieldHelper;
        $s.__get_field_helper::<$crate::ident!($f), _, _>()
    }};
    (&mut $s:ident . $f:ident) => {{
        use $crate::__GetFieldHelper;
        $s.__get_field_mut_helper::<$crate::ident!($f), _, _>()
    }};
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
        use $crate::__tuplez::TupleLike;
        $e.as_ref().foreach($crate::__tuplez::mapper! {
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
        use $crate::__tuplez::TupleLike;
        $e.as_mut().foreach($crate::__tuplez::mapper! {
            <'a, T: Copy, U> | x: &'a mut (T, U) | -> (T, &'a mut U) {
                (x.0, &mut x.1)
            }
        })
    }};
}

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
#[macro_export]
macro_rules! stru {
    ($($t:tt)*) => {
        $crate::stru_inner!($crate; $($t)*)
    };
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
#[macro_export]
macro_rules! stru_t {
    ($($t:tt)*) => {
        $crate::stru_t_inner!($crate; $($t)*)
    };
}
