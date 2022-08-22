# ref_kind

[![Crate](https://img.shields.io/crates/v/ref_kind.svg)](https://crates.io/crates/ref_kind)
[![Docs](https://docs.rs/ref_kind/badge.svg)](https://docs.rs/ref_kind)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg)

This crate provides 2 kinds of reference: immutable and mutable.
All of them represented in one enum `RefKind`, which allows to store immutable and mutable references together.

But the most importantly, this crate allows to retrieve **many** mutable references
out of the collection by creating a new collection which holds these references.

For that very case, crate defines `Many` trait which is implemented
for arrays and slices of `Option<RefKind<'a, T>>` elements.

But nothing stops you to implement this trait for other collections as well!

## Example

```rust
use core::array;

use ref_kind::{Many, RefKind};

// Create an array of square of integers from 0 to 9
let mut array: [_; 10] = array::from_fn(|i| i * i);

// Create vector of mutable references on all of the array elements
let mut many = array
    .iter_mut()
    .map(|r#mut| Some(RefKind::Mut(r#mut)))
    .collect::<Vec<_>>();

// Move out mutable reference by index 1
// It is no longer in the vector
let one = many.move_mut(1).unwrap();
assert_eq!(*one, 1);

// Move out immutable reference by index 4
// Vector now contains immutable reference, not mutable one
let four = many.move_ref(4).unwrap();
assert_eq!(*four, 16);
// Move it again: no panic here because immutable reference was copied
let four_again = many.move_ref(4).unwrap();
assert_eq!(four, four_again);

// This call will return an error because vector contains no reference by index 1
let one_again = many.try_move_ref(1);
assert!(one_again.is_err());
```

This crate used to be the part of `toucan_ecs` crate,
but now was moved into the separate crate!

## `#![no_std]` support

This crate is a `no_std` crate. It depends only on the `core` crate.

## `#![forbid(unsafe_code)]`

This crate contains no `unsafe` code.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
