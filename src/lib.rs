#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Different reference kinds in Rust.
//!
//! This crate provides 2 kinds of reference: [immutable](RefKind::Ref) and [mutable](RefKind::Mut).
//! All of them represented in one enum [`RefKind`], which allows to store immutable and mutable references together.
//!
//! But the most importantly, this crate allows to retrieve **many** mutable references
//! out of the collection by creating a new collection which holds these references.
//!
//! For that very case, crate defines [`Many`] trait which is implemented
//! for slices of `Option<RefKind<'a, T>>` elements.
//!
//! But nothing stops you to implement this trait for other collections as well!
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
//!     .map(|r#mut| Some(RefKind::Mut(r#mut)))
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
//! ## `#![forbid(unsafe_code)]`
//!
//! This crate contains no `unsafe` code.

pub use kind::RefKind;
pub use many::{Many, MoveError, Result};

#[cfg(feature = "alloc")]
mod alloc;
mod kind;
mod many;
mod slice;
#[cfg(feature = "std")]
mod std;
