# structz

Anonymous struct implementation in rust.

## Overview

### Import

Basically, structz only has procedural macros, which references types from
[tuplez](https://docs.rs/tuplez) and [stringz](https://docs.rs/stringz),
so import them all:

```bash
cargo add structz tuplez stringz
```

### Create & access

```rust
use structz::*;

let age = 26;
let mut person = stru! {
    name: "John Doe",
    age,    // captures the value of the variable with the same name
    tags: vec!["developer", "rustacean"],
};

// immutable borrow
assert_eq!(field!(&person.name), &"John Doe");

// mutable borrow
*field!(&mut person.age) += 1;
assert_eq!(field!(&person.age), &27);

// consume the struct and get the field value
let tags = field!(person.tags);
assert_eq!(tags, vec!["developer", "rustacean"]);

// `person` cannot be used anymore.
```

**NOTE**: anonymous structs with the same fields but different field orders
are considered structs of the same type.

```rust
use structz::*;

let rect1 = stru! { width: 1920, height: 1080 };
let rect2 = stru! { height: 1080, width: 1920 };
assert_eq!(rect1, rect2);
```

### As argument

```rust
use structz::*;

fn print_person(person: stru_t! {name: &str, age: u8, tags: Vec<&str> }) {
    println!(
        "{} is {} years old and has tags {:?}",
        field!(&person.name),
        field!(&person.age),
        field!(&person.tags)
    );
}

let person = stru! {
    tags: vec!["programmer", "artist"],
    name: "Alice",
    age: 30,
};
print_person(person);
```

A better way is to use the `named_args` macro which helps you unpack the struct:

```rust
use structz::*;

#[named_args]
fn print_person(name: &str, age: u8, tags: Vec<&str>) {
    println!("{} is {} years old and has tags {:?}", name, age, tags);
}

let person = stru! {
    tags: vec!["programmer", "artist"],
    name: "Alice",
    age: 30,
};
print_person(person);
```

## Details

The implementation of structz is based on [stringz](https://docs.rs/stringz) and [tuplez](https://docs.rs/tuplez).

First, macros sort the input fields in lexicographic order, which ensures that anonymous structs
with the same fields but different field orders are of the same type.

Second, convert the field names into a specialized type consisting of a sequence of zero-sized types via
[stringz](https://docs.rs/stringz). Let's call them "field name types".

Finally, pack the field name type and the data type of each field, combine them into
[tuplez](https://docs.rs/tuplez)'s [`Tuple`](https://docs.rs/tuplez/latest/tuplez/struct.Tuple.html).

Since the field names is effectively replaced with a zero-sized type, it cost you nothing:

```rust
use structz::*;

assert_eq!(
    std::mem::size_of::<
        stru_t! {
            age: u8,
            name: &'static str,
            tags: Vec<&'static str>,
        },
    >(),
    std::mem::size_of::<(u8, &'static str, Vec<&'static str>)>()
);
```
