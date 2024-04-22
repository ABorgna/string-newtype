//! Trait implementations for the core types.

use bytemuck::TransparentWrapper;
use std::ops::Deref;

use super::{NewtypeBuf, NewtypeRef};

impl<Marker, T: Clone> Clone for NewtypeBuf<Marker, T> {
    fn clone(&self) -> Self {
        Self {
            s: self.s.clone(),
            _phantom: self._phantom,
        }
    }
}

impl<Marker, T: std::fmt::Debug> std::fmt::Debug for NewtypeBuf<Marker, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewtypeBuf").field("s", &self.s).finish()
    }
}

impl<Marker, T: Default> Default for NewtypeBuf<Marker, T> {
    fn default() -> Self {
        Self {
            s: Default::default(),
            _phantom: Default::default(),
        }
    }
}

impl<Marker, T: PartialEq> PartialEq for NewtypeBuf<Marker, T> {
    fn eq(&self, other: &Self) -> bool {
        self.s == other.s
    }
}

impl<Marker, T: Eq> Eq for NewtypeBuf<Marker, T> {}

impl<Marker, T: std::hash::Hash> std::hash::Hash for NewtypeBuf<Marker, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.s.hash(state);
    }
}

impl<Marker, T> Copy for NewtypeRef<Marker, T>
where
    T: Deref,
    T::Target: Copy,
{
}

impl<Marker, T> Clone for NewtypeRef<Marker, T>
where
    T: Deref,
    T::Target: Clone,
{
    fn clone(&self) -> Self {
        Self {
            _phantom: self._phantom,
            s: self.s.clone(),
        }
    }
}

impl<Marker, T> std::hash::Hash for NewtypeRef<Marker, T>
where
    T: Deref,
    T::Target: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.s.hash(state);
    }
}

impl<Marker, T> Eq for NewtypeRef<Marker, T>
where
    T: Deref,
    T::Target: Eq,
{
}

impl<Marker, T> PartialEq for NewtypeRef<Marker, T>
where
    T: Deref,
    T::Target: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.s == other.s
    }
}

impl<Marker, T> std::fmt::Debug for NewtypeRef<Marker, T>
where
    T: Deref,
    T::Target: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NewtypeRef")
            .field(&self._phantom)
            .field(&&self.s)
            .finish()
    }
}

impl<Marker, T> Deref for NewtypeBuf<Marker, T>
where
    T: Deref,
{
    type Target = NewtypeRef<Marker, T>;

    fn deref(&self) -> &Self::Target {
        let s = TransparentWrapper::peel_ref(self);
        let b: &<T as Deref>::Target = s.deref();
        TransparentWrapper::wrap_ref(b)
    }
}

impl<'a, Marker, T> From<&'a str> for NewtypeBuf<Marker, T>
where
    T: From<&'a str>,
{
    fn from(s: &'a str) -> Self {
        Self {
            s: T::from(s),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Marker, T> From<String> for NewtypeBuf<Marker, T>
where
    T: From<String>,
{
    fn from(s: String) -> Self {
        Self {
            s: T::from(s),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, Marker, T> From<&'a str> for &'a NewtypeRef<Marker, T>
where
    T: Deref<Target = str>,
{
    fn from(s: &'a str) -> Self {
        TransparentWrapper::wrap_ref(s)
    }
}

impl<'a, Marker, T> From<&'a NewtypeRef<Marker, T>> for &'a str
where
    T: Deref<Target = str>,
{
    fn from(s: &'a NewtypeRef<Marker, T>) -> Self {
        TransparentWrapper::peel_ref(s)
    }
}

impl<Marker, T, R> AsRef<R> for NewtypeBuf<Marker, T>
where
    T: AsRef<R>,
    R: ?Sized,
{
    fn as_ref(&self) -> &R {
        self.s.as_ref()
    }
}

impl<Marker, T, R> AsRef<R> for NewtypeRef<Marker, T>
where
    T: Deref,
    T::Target: AsRef<R>,
    R: ?Sized,
{
    fn as_ref(&self) -> &R {
        self.s.as_ref()
    }
}

impl<Marker, T> std::fmt::Display for NewtypeBuf<Marker, T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.s.fmt(f)
    }
}

impl<Marker, T> std::fmt::Display for NewtypeRef<Marker, T>
where
    T: Deref,
    T::Target: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.s.fmt(f)
    }
}

impl<Marker, T> std::borrow::Borrow<NewtypeRef<Marker, T>> for NewtypeBuf<Marker, T>
where
    T: Deref,
{
    fn borrow(&self) -> &NewtypeRef<Marker, T> {
        let s = TransparentWrapper::peel_ref(self);
        let b: &<T as Deref>::Target = s.deref();
        TransparentWrapper::wrap_ref(b)
    }
}
