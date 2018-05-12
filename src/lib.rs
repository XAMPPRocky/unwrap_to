#![no_std]

//! # unwrap_to
//! A simple utility macro that allows you to easily unwrap a ADT(Abstract
//! Data Type) enum into a reference of its inner type.
//! ```
//! #[macro_use] extern crate unwrap_to;
//!
//! enum Rule {
//!     String(String),
//!     Number(u64),
//! }
//!
//! fn main() {
//!     let rule = Rule::Number(7);
//!     assert_eq!(&7, unwrap_to!(rule => Rule::Number));
//! }
//! ```

//! A simple utility macro that allows you to easily unwrap a ADT(Abstract
//! Data Type) enum into a reference of its inner type.
/// ```
/// #[macro_use] extern crate unwrap_to;
///
/// enum Rule {
///     String(String),
///     Number(u64),
/// }
///
/// fn main() {
///     let rule = Rule::Number(7);
///     assert_eq!(&7, unwrap_to!(rule => Rule::Number));
/// }
/// ```
///
/// # Panics
///
/// The macro will panic as unreachable if the variant doesn't match.
///
#[macro_export]
macro_rules! unwrap_to {
    ($var:expr => $variant:path) => {
        match &$var {
            &$variant(ref v) => v,
            _ => unreachable!(),
        }
    }
}
