#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use libloading::Library;

use crate::capi::pbtypes::*;

#[repr(C)]
pub struct IVmAPI{
    lib:Library,
}

impl IVmAPI {
    pub fn load(ver:&str)->Self{
        IVmAPI { 
            lib:  unsafe{Library::new(format!("pbvm{}.dll",ver)).expect("load pbvm error")}
        }
    }
}

macro_rules! vmapidef {
    ($name:ident,$func:ty) => {     
        type $name = libloading::Symbol<'static,$func>;
        impl IVmAPI {
            pub fn $name(&'static self)->$name{
                unsafe{self.lib.get(stringify!($name).as_bytes()).unwrap()}
            }
        }
    };
}
/* args */
vmapidef!(ot_set_return_val,extern "stdcall" fn(POB_THIS,POB_DATA)->VOID);
vmapidef!(ot_no_return_val,extern "stdcall" fn(POB_THIS)->VOID);
vmapidef!(ot_get_next_evaled_arg_no_convert,extern "stdcall" fn(POB_THIS)->POB_DATA);
vmapidef!(ot_get_valptr_arg,extern "stdcall" fn(POB_THIS,&mut BOOL)->PVOID);
vmapidef!(ot_get_next_lvalue_arg,extern "stdcall" fn(POB_THIS,POT_LVALUE_INFO)->POB_DATA);

/* dup */
vmapidef!(ob_dup_string,extern "stdcall" fn(POB_THIS,LPTSTR)->LPTSTR);
vmapidef!(ob_dup_blob,extern "stdcall" fn(POB_THIS,PSH_BINARY)->PSH_BINARY);
vmapidef!(ob_dup_dec,extern "stdcall" fn(POB_THIS,PSH_DEC)->PSH_DEC);
vmapidef!(ob_dup_double,extern "stdcall" fn(POB_THIS,PDOUBLE)->PDOUBLE);
vmapidef!(ob_dup_longlong,extern "stdcall" fn(POB_THIS,PLONGLONG)->PLONGLONG);
vmapidef!(ob_dup_time,extern "stdcall" fn(POB_THIS,PSH_TIME)->PSH_TIME);

/* alloc */
vmapidef!(ob_alloc_string,extern "stdcall" fn(POB_THIS,ULONG)->LPTSTR);
vmapidef!(ob_alloc_blob,extern "stdcall" fn(POB_THIS,ULONG)->PSH_BINARY);
vmapidef!(ob_alloc_dec,extern "stdcall" fn(POB_THIS)->PSH_DEC);
vmapidef!(ob_alloc_double,extern "stdcall" fn(POB_THIS)->PDOUBLE);
vmapidef!(ob_alloc_longlong,extern "stdcall" fn(POB_THIS)->PLONGLONG);
vmapidef!(ob_alloc_time,extern "stdcall" fn(POB_THIS)->PSH_TIME);

/* realloc */
vmapidef!(ob_realloc_string,extern "stdcall" fn(POB_THIS,LPTSTR,ULONG)->LPTSTR);
vmapidef!(ob_realloc_blob,extern "stdcall" fn(POB_THIS,PSH_BINARY,ULONG)->PSH_BINARY);

/* free */
vmapidef!(ob_free_value,extern "stdcall" fn(POB_THIS,PVOID)->VOID);
vmapidef!(ob_free_memory,extern "stdcall" fn(POB_THIS,PVOID)->VOID);

/* array */
vmapidef!(ot_array_create_bounded,extern "stdcall" fn(POB_THIS,ULONG,OB_CLASS_HNDL,USHORT,USHORT,PLONG)->PVOID);
vmapidef!(ot_array_create_unbounded,extern "stdcall" fn(POB_THIS,OB_CLASS_HNDL,USHORT)->PVOID);
vmapidef!(ot_array_num_items,extern "stdcall" fn(POB_THIS,PVOID)->ULONG);
vmapidef!(ot_array_index,extern "stdcall" fn(POB_THIS,PVOID,ULONG)->POB_DATA);

vmapidef!(ot_array_set_free_data,extern "stdcall" fn(POB_THIS,PVOID,BOOL)->VOID);
vmapidef!(ot_array_free_data,extern "stdcall" fn(POB_THIS,PVOID)->BOOL);
vmapidef!(ot_array_class_id,extern "stdcall" fn(POB_THIS,PVOID)->OB_CLASS_HNDL);
vmapidef!(ot_array_class_hndl,extern "stdcall" fn(POB_THIS,PVOID)->ULONG);
vmapidef!(ot_array_num_dimensions,extern "stdcall" fn(POB_THIS,PVOID)->USHORT);

vmapidef!(ot_is_array_unbounded,extern "stdcall" fn(POB_THIS,PVOID)->BOOL);
vmapidef!(ot_get_arraydef_no_dims,extern "stdcall" fn(POB_THIS,PVOID)->USHORT);
vmapidef!(ot_get_arraydef_style,extern "stdcall" fn(POB_THIS,PVOID)->OB_ARRAY_SYMBOL_STYLE);
vmapidef!(ot_get_arraydef_bounds,extern "stdcall" fn(POB_THIS,PVOID)->PLONG);
vmapidef!(ot_get_arraydef_varinfo,extern "stdcall" fn(POB_THIS,PVOID)->OB_INFO_FLAGS);
vmapidef!(ot_get_arraydef_upper_bound,extern "stdcall" fn(POB_THIS,PVOID,LONG)->LONG);
vmapidef!(ot_get_arraydef_lower_bound,extern "stdcall" fn(POB_THIS,PVOID,LONG)->LONG);

/* obj */
vmapidef!(ot_get_curr_obinst_expr,extern "stdcall" fn(POB_THIS,POB_INST_ID,PBOOL)->POB_INST_ID);