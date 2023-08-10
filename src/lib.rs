#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Different reference kinds in Rust.
//!
//! This crate provides 2 kinds of reference: [immutable](Ref) and [mutable](Mut).
//! All of them represented in one enum [`RefKind`], which allows to store immutable and mutable references together.
//!
//! But the most importantly, this crate allows to retrieve **many** mutable references
//! out of the collection by creating a new collection which holds these references.
//!
//! For that very case, crate defines some useful traits:
//! - [`MoveRef`] and [`MoveMut`] for containers to retrieve corresponding kind of reference,
//! - [`Move`] as a combination of the traits above,
//! - [`Many`] for collections which is implemented for peekable iterators, [slices] and so on.
//!
//! But nothing stops you to implement these traits for other types as well!
//!
//! ## Example
//!
//! ```
//! use core::array;
//!
//! use ref_kind::{Many, RefKind, MoveError};
//!
//! // Create an array of square of integers from 0 to 9
//! let mut array: [_; 10] = array::from_fn(|i| i * i);
//!
//! // Create collection of mutable references on all of the array elements
//! let mut many: [_; 10] = array
//!     .iter_mut()
//!     .map(|sq| Some(RefKind::from(sq)))
//!     .collect::<Vec<_>>()
//!     .try_into()
//!     .unwrap();
//!
//! // Move out mutable reference by index 1
//! // It is no longer in the `many`
//! let one = many.move_mut(1).unwrap();
//! assert_eq!(*one, 1);
//!
//! // Move out immutable reference by index 4
//! // `many` now contains immutable reference, not mutable one
//! let four = many.move_ref(4).unwrap();
//! assert_eq!(*four, 16);
//! // Move it again: no panic here because immutable reference was copied
//! let four_again = many.move_ref(4).unwrap();
//! assert_eq!(four, four_again);
//!
//! // This call will return an error because `many` contains no reference by index 1
//! let one_again = many.try_move_ref(1);
//! assert_eq!(one_again, Err(MoveError::BorrowedMutably));
//! ```
//!
//! ## `#![no_std]` support
//!
//! This crate is a `no_std` crate. It depends only on the `core` crate.
//!
//! `std` feature of the crate is enabled by default, so to use it in `no_std` environment,
//! you should disable default features of this crate in Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! ref_kind = { version = "0.5.0", default-features = false }
//! ```
//!
//! ## `#![forbid(unsafe_code)]`
//!
//! This crate contains no `unsafe` code.
//!
//! ## Flags
//!
//! This crate has the following Cargo features:
//!
//! | Feature name | Description                                                                           |
//! |--------------|---------------------------------------------------------------------------------------|
//! | `alloc`      | Implements `Many` trait for `VecDeque` and `BTreeMap` in `alloc` crate                |
//! | `std`        | Implements `Many` trait for `HashMap` in standard library, depends on `alloc` feature |
//! | `hashbrown`  | Implements `Many` trait for `HashMap` in `hashbrown` crate                            |
//!
//! Feature `std` is enabled by default.
//! You can disable it by using `default-features = false` in Cargo.toml.
//!
//! These features were added to this crate to make it usable
//! with common Rust collections, such as [`Vec`] and [`HashMap`].
//!
//! [slices]: prim@slice
//! [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
//! [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html

#[cfg(feature = "alloc")]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std as std_crate;

pub use self::{
    kind::RefKind,
    many::Many,
    r#move::{Move, MoveError, MoveMut, MoveRef, Result},
    RefKind::{Mut, Ref},
};

pub mod iter;

#[cfg(feature = "alloc")]
mod alloc;
#[cfg(feature = "hashbrown")]
mod hashbrown;
mod kind;
mod many;
mod r#move;
mod slice;
#[cfg(feature = "std")]
mod std;
