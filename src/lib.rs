//! # anydata
//!
//! This crate is used to parse an unknown DateTime or Date format into a normalized version.
//!
//! Any significant changes to anydate are documented in
//! the [`CHANGELOG.md`](https://github.com/rust-playground/anydate/blob/main/CHANGELOG.md) file.
//!
//! ## Usage
//! ```toml
//! [dependencies]
//! anydate = "0.1"
//! ```
//!
//! ### Features
//!
//! Optional features:
//!
//! - [`serde`][]: Enable deserialize_with helper functions via serde.
//!
//! [`serde`]: https://github.com/serde-rs/serde
//!

pub mod date;
pub mod datetime;
pub mod errors;
#[cfg(feature = "serde")]
pub mod serde;

#[doc(inline)]
pub use datetime::{parse, parse_utc};
