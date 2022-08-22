#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Different reference kinds in Rust.
//!
//! Provides 2 kinds of reference: [immutable](RefKind::Ref) and [mutable](RefKind::Mut).
//! All of them represented in one enum [`RefKind`], which allows to store immutable and mutable references together.
//!
//! TODO: replace custom types with trait which will add functionality to existing types
//!
//! ## `#![no_std]` support
//!
//! This crate is a `no_std` crate. It depends only on the `core` crate.
//!
//! ## `#![forbid(unsafe_code)]`
//!
//! This crate contains no `unsafe` code.

pub use kind::RefKind;

mod kind;
