#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

//! Different reference kinds in Rust.
//!
//! Provides 2 kinds of reference: immutable and mutable. All of them represented in one enum [`RefKind`],
//! which allows you to store immutable and mutable references together.
//!
//! In addition, this crate contains [`RefKindMap`] which is a [`HashMap`] of reference kinds.
//! This structure can easily be created from [`HashMap`] iterator (immutable or mutable one):
//!
//! ```
//! use std::collections::HashMap;
//! use ref_kind::RefKindMap;
//!
//! let mut map = HashMap::new();
//! map.insert("Hello World", 0);
//! map.insert("The Answer to the Ultimate Question of Life, the Universe, and Everything", 42);
//!
//! let mut refs = map.iter_mut().map(|(&k, v)| (k, v)).collect::<RefKindMap<_, _>>();
//! ```
//!
//! Then it can be used to retrieve multiple mutable references from the [`HashMap`]:
//!
//! ```
//! # use std::collections::HashMap;
//! # use ref_kind::RefKindMap;
//! #
//! # let mut map = HashMap::new();
//! # map.insert("Hello World", 0);
//! # map.insert("The Answer to the Ultimate Question of Life, the Universe, and Everything", 42);
//! #
//! # let mut refs = map.iter_mut().map(|(&k, v)| (k, v)).collect::<RefKindMap<_, _>>();
//! #
//! let hello = refs.move_mut("Hello World").unwrap();
//! let answer = refs.move_mut("The Answer to the Ultimate Question of Life, the Universe, and Everything").unwrap();
//!
//! assert_eq!(*hello, 0);
//! assert_eq!(*answer, 42);
//! ```
//!
//! No `unsafe` code is needed!
//!
//! [`HashMap`]: std::collections::HashMap

pub use kind::RefKind;
pub use map::RefKindMap;

#[cfg(feature = "bumpalo")]
pub mod bumpalo;
mod kind;
mod map;
