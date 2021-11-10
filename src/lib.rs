//! # anydata
//!
//! This crate is used to parse an unknown DateTime or Date format into a normalized version.
//!
//! ```rust
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // see parse_utc() for convenience conversion to UTC
//!     let parsed = anydate::parse("2021-11-10T03:25:06.533447000Z")?;
//!     println!("{:#?}", parsed);
//!     Ok(())
//! }
//! ```
//!
//! or if you know it's only a date with no time component
//!
//! ```rust
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let parsed = anydate::date::parse("2021-11-10");
//!     println!("{:#?}", parsed);
//!     Ok(())
//! }

pub mod date;
pub mod datetime;
pub mod errors;

#[doc(inline)]
pub use datetime::{parse, parse_utc};
