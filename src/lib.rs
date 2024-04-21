//! # string-newtype: New Type idiom helper for string-like types
//!
//!
//! `string-newtype` is a helper library for using the [newtype idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) with string-like types, including newtyped string slices.
//!
//! # Usage
//!
//! Define an empty enum as a marker for your type, and add aliases based on it:
//!
//! ```rust
//! # use string_newtype::{StringBuf, StringRef};
//! // A marker type, can be anything.
//! enum S {}
//!
//! // The newtype definitions.
//! // `StringBuf` acts as a `String`, while `StringRef` acts as a `str`.
//! type SBuf = StringBuf<S>;
//! type SRef = StringRef<S>;
//!
//! // Define functions that only accept the newtype.
//! fn my_func(owned: SBuf, reference: &SRef) {
//!     // ...
//! }
//!
//! // Only the newtype can be passed to the function.
//! let s: SBuf = "hello".into();
//! my_func(s.clone(), &s);
//! ```

mod core;
#[cfg(feature = "smol_str")]
mod smol_str;
mod string;

pub use crate::core::{NewtypeBuf, NewtypeRef};
#[cfg(feature = "smol_str")]
pub use crate::smol_str::{SmolStrBuf, SmolStrRef};
pub use crate::string::{StringBuf, StringRef};
