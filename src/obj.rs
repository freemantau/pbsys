use std::ptr::NonNull;

use crate::refv::Pvoid;

pub type Pob_Inst_Id = Option<Pvoid>;

#[repr(C)]
pub struct _PBCLASS([u8;0]);
pub type pobclass = NonNull<_PBCLASS>;
pub struct ObClass{
    ptr:pobclass
}
impl ObClass {
    pub fn from_ptr(ptr:pobclass)->Self{
        ObClass { ptr: ptr }
    }
    pub fn as_ptr(&self)->pobclass{
        self.ptr
    }
}

impl Default for ObClass {
    fn default() -> Self {
        Self { 
            ptr:NonNull::<_PBCLASS>::dangling() 
         }
    }
}