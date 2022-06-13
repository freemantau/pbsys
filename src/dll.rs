use crate::arr::Parray;
use crate::pbsys::*;
use crate::refv::*;
use crate::obj::*;
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
    /// 设置返回
    /// 
    pub static ref OT_SET_RETURN_VAL:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&ObData)> = unsafe{CONTEXT.vm.get(b"ot_set_return_val").unwrap()};
    ///
    /// 无返回值
    /// 
    pub static ref OT_NO_RETURN_VAL:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm)> = unsafe{CONTEXT.vm.get(b"ot_no_return_val").unwrap()};
    ///
    /// pbvmxxx.dll
    /// 获取参数值/指针
    pub static ref OT_GET_VALPTR_ARG:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&bool) -> *mut u8 > = unsafe{CONTEXT.vm.get(b"ot_get_valptr_arg").unwrap()};
    ///
    /// 值类型  obdata
    /// 
    pub static ref OT_GET_NEXT_EVALED_ARG_NO_CONVERT:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm) -> *mut ObData> = unsafe{CONTEXT.vm.get(b"ot_get_next_evaled_arg_no_convert").unwrap()};
    
    ///
    /// 引用类型 再根据obdata::val指针获取引用包refpak
    /// 
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

    ///
    /// array
    ///
    pub static ref OT_ASSIGN_LVALUE_ARRAY:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&mut ObData,Parray,&mut bool)> = unsafe {CONTEXT.vm.get(b"ot_assign_lvalue_array").unwrap()};

    ///
    /// 获取值arraynum
    ///
    pub static ref OT_ARRAY_NUM_ITEMS:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,Parray)->u32> = unsafe{CONTEXT.vm.get(b"ot_array_num_items").unwrap()};

    ///
    /// 单个array值
    ///
    pub static ref OT_ARRAY_INDEX:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,Parray,u32)->*mut ObData> = unsafe{CONTEXT.vm.get(b"ot_array_index").unwrap()};

    ///
    /// bounded array
    ///
    pub static ref OT_ARRAY_CREATE_BOUNDED:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,u32,Ob_Class_Hndl,u16,u16,Pvoid)->Parray> = unsafe{CONTEXT.vm.get(b"ot_array_create_bounded").unwrap()};

    ///
    /// unbounded array
    ///
    pub static ref OT_ARRAY_CREATE_UNBOUNDED:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,Ob_Class_Hndl,u16)->Parray> = unsafe{CONTEXT.vm.get(b"ot_array_create_unbounded").unwrap()};

    ///
    /// assign ref data
    ///
    pub static ref OT_ASSIGN_REF_INT:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,i32,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_int)").unwrap()};
    pub static ref OT_ASSIGN_REF_UINT:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,u32,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_uint)").unwrap()};
    pub static ref OT_ASSIGN_REF_BYTE:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,u8,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_byte)").unwrap()};
    pub static ref OT_ASSIGN_REF_LONG:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,i32,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_long)").unwrap()};
    pub static ref OT_ASSIGN_REF_ULONG:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,u32,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_ulong)").unwrap()};
    pub static ref OT_ASSIGN_REF_DEC:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,&Psh_Dec,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_dec)").unwrap()};
    pub static ref OT_ASSIGN_REF_FLOAT:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,f32,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_float)").unwrap()};
    pub static ref OT_ASSIGN_REF_DOUBLE:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,&f64,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_double)").unwrap()};
    pub static ref OT_ASSIGN_REF_LONGLONG:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,&i64,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_longlong)").unwrap()};
    pub static ref OT_ASSIGN_REF_STRING:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,&[u16],bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_string)").unwrap()};
    pub static ref OT_ASSIGN_REF_BOOL:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,bool,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_bool)").unwrap()};
    pub static ref OT_ASSIGN_REF_CHAR:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,u16,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_char)").unwrap()};
    pub static ref OT_ASSIGN_REF_BLOB:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,&Psh_Binary,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_blob)").unwrap()};
    pub static ref OT_ASSIGN_REF_DATETIME:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,&Psh_Time,bool)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_datetime)").unwrap()};
    pub static ref OT_ASSIGN_REF_OBINST:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,Pvoid,bool,u16)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_obinst)").unwrap()};
    pub static ref OT_ASSIGN_REF_ENUM:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,i32,bool,u16)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_enum)").unwrap()};
    pub static ref OT_ASSIGN_REF_ARRAY:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,Parray,bool,u16)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_array)").unwrap()};
    pub static ref OT_ASSIGN_REF_ANY:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&Ot_Ref_Pak,&ObData,u16)> = unsafe{CONTEXT.vm.get(b"ot_assign_ref_any)").unwrap()};


    ///
    /// 当前实例指针
    /// 
    pub static ref OT_GET_CURR_OBINST_EXPR:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,&ObClass,&mut bool)->Pvoid> = unsafe{CONTEXT.vm.get(b"ot_get_curr_obinst_expr").unwrap()};

    ///
    /// 
    /// 
    pub static ref OB_SET_FIELD:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,pobclass,u32,&ObData)> = unsafe{CONTEXT.vm.get(b"ob_set_field").unwrap()};
    pub static ref OB_GET_FIELD:libloading::Symbol<'static,unsafe extern "stdcall" fn(pobvm,pobclass,u32,&mut ObData)> = unsafe{CONTEXT.vm.get(b"ob_get_field").unwrap()};


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
