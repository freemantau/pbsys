use crate::pbsys::*;

use lazy_static::lazy_static;

use libloading::Library;

/* 
pbstg_alloc
pbstg_realloc
pbstg_free
*/

lazy_static!{
    pub static ref CONTEXT:LibContext = {
        LibContext::init()
    };
    pub static ref OT_GET_VALPTR_ARG:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&bool) -> *mut u8 > = unsafe{CONTEXT.vm.get(b"ot_get_valptr_arg").unwrap()};
    pub static ref OT_SET_RETURN_VAL:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&ObData)> = unsafe{CONTEXT.vm.get(b"ot_set_return_val").unwrap()};
    pub static ref OT_GET_NEXT_EVALED_ARG_NO_CONVERT:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm) -> *mut ObData> = unsafe{CONTEXT.vm.get(b"ot_get_next_evaled_arg_no_convert").unwrap()};
}


pub struct LibContext
{
    vm:Library,
    shr:Library,
    dwe:Library,
    sys:Library
}

impl LibContext {
    fn init()->Self{
        LibContext{
            vm:unsafe{libloading::Library::new("pbvm125.dll").unwrap()},
            shr:unsafe{libloading::Library::new("pbshr125.dll").unwrap()},
            dwe:unsafe{libloading::Library::new("pbdwe125.dll").unwrap()},
            sys:unsafe{libloading::Library::new("pbsys125.dll").unwrap()}
        }
    }
}