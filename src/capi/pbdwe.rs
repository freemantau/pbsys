#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use libloading::Library;

#[repr(C)]
pub struct IDweAPI{
    lib:Library
}
impl IDweAPI {
    pub fn load(ver:&str)->Self{
        IDweAPI { 
            lib:  unsafe{Library::new(format!("pbdwe{}.dll",ver)).expect("load pbdew error")}
        }
    }
}

macro_rules! dweapidef {
    ($name:ident,$func:ty) => {     
        pub type $name = libloading::Symbol<'static,$func>;
        impl IDweAPI {
            fn $name(&'static self)->$name{
                unsafe{self.lib.get(stringify!($name).as_bytes()).unwrap()}
            }
        }
    };
}