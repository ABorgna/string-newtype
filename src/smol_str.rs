use smol_str::SmolStr;

use crate::core::{NewtypeBuf, NewtypeRef};

pub type SmolStrBuf<Marker> = NewtypeBuf<Marker, SmolStr>;
pub type SmolStrRef<Marker> = NewtypeRef<Marker, SmolStr>;

impl<Marker> SmolStrBuf<Marker> {
    /// Constructs an inline variant.
    ///
    /// # Panics
    /// If the length of `text` is greater than 23.
    pub const fn new_inline(text: &str) -> Self {
        Self {
            s: SmolStr::new_inline(text),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Construct a new buffer from a statically allocated string.
    ///
    /// This never allocates.
    pub const fn new_static(text: &'static str) -> Self {
        Self {
            s: SmolStr::new_static(text),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Construct a new buffer from a string.
    pub fn new<T>(text: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            s: SmolStr::new(text),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns this buffer as a string slice.
    pub fn as_str(&self) -> &str {
        self.s.as_str()
    }

    /// Returns the length of this buffer.
    pub fn len(&self) -> usize {
        self.s.len()
    }

    /// Returns `true` if this buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }

    /// Returns `true` if this buffer is heap-allocated.
    pub const fn is_heap_allocated(&self) -> bool {
        self.s.is_heap_allocated()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smolstr_newtype() {
        enum S {}
        type SBuf = SmolStrBuf<S>;
        type SRef = SmolStrRef<S>;

        let s1: SBuf = SBuf::new_inline("Hello");
        let s2: SBuf = SBuf::new_inline("Hello");
        let _s3: SBuf = SBuf::new(s2.as_str());

        let s_ref: &SRef = &s1;
        let hello: &SRef = "Hello".into();
        assert_eq!(hello, s_ref);

        assert_eq!(s2.len(), 5);
        assert!(!s2.is_empty());
        assert!(!s2.is_heap_allocated());
    }

    #[test]
    fn asref() {
        fn f(_: impl AsRef<str>) {}

        let s: SmolStrBuf<()> = SmolStrBuf::new("Hello");
        let s_ref: &SmolStrRef<()> = &s;

        f(&s);
        f(s_ref);
    }

    #[test]
    fn tostring() {
        let s: SmolStrBuf<()> = SmolStrBuf::new("Hello");
        let s_ref: &SmolStrRef<()> = &s;

        assert_eq!(s.to_string(), "Hello");
        assert_eq!(s_ref.to_string(), "Hello");
    }

    #[test]
    fn toowned() {
        let s: SmolStrBuf<()> = SmolStrBuf::new("Hello");
        let s_ref: &SmolStrRef<()> = &s;

        let owned: SmolStrBuf<()> = SmolStrRef::to_owned(&s_ref);

        assert_eq!(owned, s);
    }
}
