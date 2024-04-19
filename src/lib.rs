//! # string-newtype: New Type idiom helper for string-like types

mod core;
#[cfg(feature = "smol_str")]
mod smol_str;
mod string;

pub use crate::core::{NewtypeBuf, NewtypeRef};
#[cfg(feature = "smol_str")]
pub use crate::smol_str::{SmolStrBuf, SmolStrRef};
pub use crate::string::{StringBuf, StringRef};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_newtype() {
        type S = StringBuf<()>;
        type SRef = StringRef<()>;

        let s: S = "Hello".into();
        let s_ref: &SRef = &s;
        let hello: &SRef = "Hello".into();
        assert_eq!(hello, s_ref);
    }
}
