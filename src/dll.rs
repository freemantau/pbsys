use crate::pbsys::*;

use lazy_static::lazy_static;

use libloading::Library;

lazy_static!{
    pub static ref CONTEXT:LibContext = {
        let lib = unsafe{libloading::Library::new("pbvm125.dll").unwrap()};
        //let ot_get_valptr_arg:libloading::Symbol<unsafe extern fn(*mut _POBVM,bool) -> *const u8 > = unsafe{lib.get(b"ot_get_valptr_arg").unwrap()};
        return LibContext { lib: lib }
    };
    pub static ref OT_GET_VALPTR_ARG:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&bool) -> *mut u8 > = unsafe{CONTEXT.lib.get(b"ot_get_valptr_arg").unwrap()};
    pub static ref OT_SET_RETURN_VAL:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&ObData)> = unsafe{CONTEXT.lib.get(b"ot_set_return_val").unwrap()};
    pub static ref OT_GET_NEXT_EVALED_ARG_NO_CONVERT:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm) -> *mut ObData> = unsafe{CONTEXT.lib.get(b"ot_get_next_evaled_arg_no_convert").unwrap()};
}


pub struct LibContext
{
    pub lib:Library,
}
