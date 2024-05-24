# stringz

Convert strings to types to make it available as generic parameters.

## Install

```bash
cargo add stringz tuplez
```

## Example

```rust
use stringz::{TypedString, string};

fn test_hello<T: TypedString>() {
    assert_eq!(T::value(), "hello");
}

test_hello::<string!("hello")>();
```

## Explanation

The `string` macro converts `"hello"` to the following tuple type:

```text
(Character<'h'>, Character<'e'>, Character<'l'>, Character<'l'>, Character<'o'>)
```

Note: The above form is only for ease of understanding, the actual `Tuple` type of
[tuplez](https://docs.rs/tuplez) is used.

All generated types are zero-sized types:

```rust
use stringz::string;
assert_eq!(std::mem::size_of::<string!("no matter how long it is")>(), 0);
```
