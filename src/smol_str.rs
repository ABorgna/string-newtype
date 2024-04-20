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
