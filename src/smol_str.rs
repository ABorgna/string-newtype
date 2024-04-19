use smol_str::SmolStr;

use crate::core::{NewtypeBuf, NewtypeRef};

pub type SmolStrBuf<Marker> = NewtypeBuf<Marker, SmolStr>;
pub type SmolStrRef<Marker> = NewtypeRef<Marker, SmolStr>;

impl<Marker> SmolStrBuf<Marker> {
    pub const fn new_inline(s: &str) -> Self {
        Self {
            s: SmolStr::new_inline(s),
            _phantom: std::marker::PhantomData,
        }
    }
}
