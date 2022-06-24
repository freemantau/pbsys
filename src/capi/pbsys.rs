#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use libloading::Library;

#[repr(C)]
pub struct ISysAPI{
    lib:Library
}
impl ISysAPI {
    pub fn load(ver:&str)->Self{
        ISysAPI { 
            lib:  unsafe{Library::new(format!("pbsys{}.dll",ver)).expect("load pbsys error")}
        }
    }
}

macro_rules! sysapidef {
    ($name:ident,$func:ty) => {     
        pub type $name = libloading::Symbol<'static,$func>;
        impl ISysAPI {
            fn $name(&'static self)->$name{
                unsafe{self.lib.get(stringify!($name).as_bytes()).unwrap()}
            }
        }
    };
}