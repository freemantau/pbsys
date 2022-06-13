use std::ptr::NonNull;

use crate::{refv::Pvoid, *};

pub type Parray = Pvoid;

#[repr(C)]
pub struct _POBARRAY([u8; 0]);
pub type pobarray = NonNull<_POBARRAY>;
#[repr(transparent)]
pub struct Ob_Array_Id {
    ptr: pobarray,
}
impl Ob_Array_Id {
    pub fn as_ptr(&self) -> pobarray {
        self.ptr
    }
}
