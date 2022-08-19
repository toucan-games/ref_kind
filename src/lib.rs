#![warn(missing_docs)]
#![forbid(unsafe_code)]

//! Different reference kinds in Rust.

pub use kind::RefKind;
pub use map::RefKindMap;

mod kind;
mod map;
