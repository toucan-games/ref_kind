# ref_kind

[![Crate](https://img.shields.io/crates/v/ref_kind.svg)](https://crates.io/crates/ref_kind)
[![Docs](https://docs.rs/ref_kind/badge.svg)](https://docs.rs/ref_kind)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg)

Different reference kinds in Rust.

Provides 2 kinds of reference: immutable and mutable. All of them represented in one enum `RefKind`,
which allows you to store immutable and mutable references together.

In addition, this crate contains `RefKindMap` which is a `HashMap` of reference kinds.
This structure can easily be created from `HashMap` iterator (immutable or mutable one):

```rust
use std::collections::HashMap;
use ref_kind::RefKindMap;

let mut map = HashMap::new();
map.insert("Hello World", 0);
map.insert("The Answer to the Ultimate Question of Life, the Universe, and Everything", 42);

let mut refs = map.iter_mut().map(|(&k, v)| (k, v)).collect::<RefKindMap<_, _>>();
```

Then it can be used to retrieve multiple mutable references from the `HashMap`:

```rust
let hello = refs.move_mut("Hello World").unwrap();
let answer = refs.move_mut("The Answer to the Ultimate Question of Life, the Universe, and Everything").unwrap();

assert_eq!(*hello, 0);
assert_eq!(*answer, 42);
```

This crate used to be the part of `toucan_ecs` crate,
but now was moved into the separate crate!

## `#![no_std]` support

This crate is a `no_std` crate. It depends only on the `alloc` and `core` crates.

## `#![forbid(unsafe_code)]`

This crate contains no `unsafe` code.

## Flags

This crate has the following Cargo features:

| Feature name | Description                                                       |
|--------------|-------------------------------------------------------------------|
| `bumpalo`    | Compatibility with `bumpalo` crate to be able to reuse heap space |

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
