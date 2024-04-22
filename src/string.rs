pub type StringBuf<Marker> = crate::NewtypeBuf<Marker, String>;
pub type StringRef<Marker> = crate::NewtypeRef<Marker, String>;

impl<Marker> StringBuf<Marker> {
    /// Creates a new empty buffer.
    pub fn new() -> Self {
        Self {
            s: String::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new empty buffer with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            s: String::with_capacity(capacity),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns this buffer as a string slice.
    pub fn as_str(&self) -> &str {
        self.s.as_str()
    }

    //// Converts the buffer into a mutable slice.
    pub fn as_mut_str(&mut self) -> &mut str {
        self.s.as_mut_str()
    }

    /// Returns the length of this buffer.
    pub fn len(&self) -> usize {
        self.s.len()
    }

    /// Returns `true` if this buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }
}

impl<Marker> StringRef<Marker> {
    /// Returns the length of this buffer.
    pub fn len(&self) -> usize {
        self.s.len()
    }

    /// Returns `true` if this buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_newtype() {
        enum S {}
        type SBuf = StringBuf<S>;
        type SRef = StringRef<S>;

        let s: SBuf = "Hello".into();
        let s_ref: &SRef = &s;
        let hello: &SRef = "Hello".into();
        assert_eq!(hello, s_ref);
    }

    #[test]
    fn asref() {
        fn f(_: impl AsRef<str>) {}

        let s: StringBuf<()> = "Hello".into();
        let s_ref: &StringRef<()> = &s;

        f(&s);
        f(s_ref);
    }

    #[test]
    fn tostring() {
        let s: StringBuf<()> = "Hello".into();
        let s_ref: &StringRef<()> = &s;

        assert_eq!(s.to_string(), "Hello");
        assert_eq!(s_ref.to_string(), "Hello");
    }

    #[test]
    fn toowned() {
        let s: StringBuf<()> = "Hello".into();
        let s_ref: &StringRef<()> = &s;

        let owned: StringBuf<()> = (*s_ref).to_owned();

        assert_eq!(owned, s);
    }
}
