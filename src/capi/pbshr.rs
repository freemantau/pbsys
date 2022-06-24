#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use libloading::Library;

use crate::capi::pbtypes::*;

#[repr(C)]
pub struct IShrAPI{
    lib:Library
}
impl IShrAPI {
    pub fn load(ver:&str)->Self{
        IShrAPI { 
            lib:  unsafe{Library::new(format!("pbshr{}.dll",ver)).expect("load pbshr error")}
        }
    }
}

macro_rules! shrapidef {
    ($name:ident,$func:ty) => {     
        pub type $name = libloading::Symbol<'static,$func>;
        impl IShrAPI {
            fn $name(&'static self)->$name{
                unsafe{self.lib.get(stringify!($name).as_bytes()).unwrap()}
            }
        }
    };
}

/* pbstg */
shrapidef!(ot_set_return_val,extern "stdcall" fn(POB_THIS,POB_DATA)->VOID);
