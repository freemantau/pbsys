use crate::pbsys::*;

use lazy_static::lazy_static;

use libloading::Library;

/*
pbstg_alloc
pbstg_realloc
pbstg_free
*/

lazy_static! {
    pub static ref CONTEXT:LibContext = {
        LibContext::init()
    };
    ///
    /// pbvmxxx.dll
    ///
    pub static ref OT_GET_VALPTR_ARG:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&bool) -> *mut u8 > = unsafe{CONTEXT.vm.get(b"ot_get_valptr_arg").unwrap()};
    pub static ref OT_SET_RETURN_VAL:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&ObData)> = unsafe{CONTEXT.vm.get(b"ot_set_return_val").unwrap()};
    pub static ref OT_GET_NEXT_EVALED_ARG_NO_CONVERT:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm) -> *mut ObData> = unsafe{CONTEXT.vm.get(b"ot_get_next_evaled_arg_no_convert").unwrap()};
    pub static ref OT_GET_NEXT_LVALUE_ARG:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&u32) -> *mut ObData> = unsafe{CONTEXT.vm.get(b"ot_get_next_lvalue_arg").unwrap()};

    ///
    /// dup ptr value
    ///
    pub static ref OB_DUP_STRING:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&[u16])->&[u16]> = unsafe{CONTEXT.vm.get(b"ob_dup_string").unwrap()};
    pub static ref OB_DUP_BLOB:libloading::Symbol<'static, unsafe extern "stdcall" fn(pobvm,&Psh_Binary)-> &Psh_Binary> = unsafe{CONTEXT.vm.get(b"ob_dup_blob").unwrap()};
    pub static ref OB_DUP_DEC:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Psh_Dec)->&Psh_Dec> = unsafe{CONTEXT.vm.get(b"ob_dup_dec").unwrap()};
    pub static ref OB_DUP_DOUBLE:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&f64) ->&f64> = unsafe{CONTEXT.vm.get(b"ob_dup_double").unwrap()};
    pub static ref OB_DUP_LONGLONG:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&i64)->&i64> = unsafe{CONTEXT.vm.get(b"ob_dup_longlong").unwrap()};
    pub static ref OB_DUP_TIME:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Psh_Time)->&Psh_Time> = unsafe{CONTEXT.vm.get(b"ob_dup_time").unwrap()};

}

pub struct LibContext {
    vm: Library,
    shr: Library,
    dwe: Library,
    sys: Library,
}

impl LibContext {
    fn init() -> Self {
        LibContext {
            vm: unsafe { libloading::Library::new("pbvm125.dll").unwrap() },
            shr: unsafe { libloading::Library::new("pbshr125.dll").unwrap() },
            dwe: unsafe { libloading::Library::new("pbdwe125.dll").unwrap() },
            sys: unsafe { libloading::Library::new("pbsys125.dll").unwrap() },
        }
    }
}
