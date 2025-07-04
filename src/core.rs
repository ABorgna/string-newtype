//! Newtype wrapper for strings

mod traits;

use bytemuck::TransparentWrapper;
use std::ops::Deref;

/// Newtype wrapper for strings
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct NewtypeBuf<Marker, T = String> {
    pub(crate) s: T,
    pub(crate) _phantom: std::marker::PhantomData<Marker>,
}

/// A slice for a [`NewtypeBuf`]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct NewtypeRef<Marker, T>
where
    T: Deref,
{
    pub(crate) _phantom: std::marker::PhantomData<Marker>,
    #[cfg_attr(
        feature = "serde",
        serde(bound(deserialize = "<T as Deref>::Target: serde::Deserialize<'de> + Sized"))
    )]
    #[cfg_attr(
        feature = "serde",
        serde(bound(serialize = "<T as Deref>::Target: serde::Serialize + Sized"))
    )]
    pub(crate) s: <T as Deref>::Target,
}

unsafe impl<Marker, T> TransparentWrapper<<T as Deref>::Target> for NewtypeRef<Marker, T> where
    T: Deref
{
}

unsafe impl<Marker, T> TransparentWrapper<T> for NewtypeBuf<Marker, T> {}
