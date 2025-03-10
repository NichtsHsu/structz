use crate::TypedString;
use crate::__tuplez::{search::Search, Tuple};

/// A trait that indicate that an anonymous struct contains a certain field.
///
/// # Generic parameters
///
/// * `Field`: Typed field name, see [`stringz::ident!`](https://docs.rs/stringz/0.1.2/stringz/macro.ident.html).
/// * `T`: The type of data carried by the field.
/// * `R`: Type used to indicate the position of the field in the struct.
///   Usually automatically inferred by Rust.
///
/// # Example
///
/// See the section ["as generic type"](structz#as-generic-type).
pub trait HasField<Field, T, R>
where
    Field: TypedString,
{
    /// Get the immutable reference to the data carried by the field.
    fn get_field<'a>(&'a self) -> &'a T
    where
        Field: 'a;

    /// Get the mutable reference to the data carried by the field.
    fn get_field_mut<'a>(&'a mut self) -> &'a mut T
    where
        Field: 'a;

    /// Consume the struct and take the data carried by the field.
    fn take_field(self) -> T;
}

impl<Field, T, R, First, Other> HasField<Field, T, R> for Tuple<First, Other>
where
    Field: TypedString,
    Self: Search<(Field, T), R>,
{
    fn get_field<'a>(&'a self) -> &'a T
    where
        Field: 'a,
    {
        &Search::get_ref(self).1
    }

    fn get_field_mut<'a>(&'a mut self) -> &'a mut T
    where
        Field: 'a,
    {
        &mut Search::get_mut(self).1
    }

    fn take_field(self) -> T {
        Search::take(self).0 .1
    }
}

/// Helper trait used for [`field!`] macro.
#[doc(hidden)]
pub trait __GetFieldHelper {
    #[doc(hidden)]
    fn __get_field_helper<'a, Field, T, R>(&'a self) -> &'a T
    where
        Field: TypedString + 'a,
        Self: HasField<Field, T, R>,
    {
        self.get_field()
    }

    #[doc(hidden)]
    fn __get_field_mut_helper<'a, Field, T, R>(&'a mut self) -> &'a mut T
    where
        Field: TypedString + 'a,
        Self: HasField<Field, T, R>,
    {
        self.get_field_mut()
    }
}

impl<T> __GetFieldHelper for T {}
