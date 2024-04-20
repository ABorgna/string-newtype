//! Newtype wrapper for strings

mod traits;

use bytemuck::TransparentWrapper;
use std::ops::Deref;

/// Newtype wrapper for strings
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct NewtypeBuf<Marker, T = String> {
    pub(crate) s: T,
    pub(crate) _phantom: std::marker::PhantomData<Marker>,
}

/// A slice for a [`NewtypeBuf`]
#[repr(transparent)]
#[cfg_attr(serde, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(serde, serde(transparent))]
pub struct NewtypeRef<Marker, T>
where
    T: Deref,
{
    pub(crate) _phantom: std::marker::PhantomData<Marker>,
    pub(crate) s: <T as Deref>::Target,
}

unsafe impl<Marker, T> TransparentWrapper<<T as Deref>::Target> for NewtypeRef<Marker, T> where
    T: Deref
{
}

unsafe impl<Marker, T> TransparentWrapper<T> for NewtypeBuf<Marker, T> {}
