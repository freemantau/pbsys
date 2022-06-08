use crate::*;


#[no_mangle]
pub unsafe extern "stdcall" fn bit_or(obthis:ObVm,nargs:u32)->u32{

    let arg1 = obthis.get_next_arg().unwrap().get_long_unchecked();
    let arg2 = obthis.get_next_arg().unwrap().get_long_unchecked();
    let _ = obthis.set_return_long(arg1 | arg2);
    return 1;
}



#[no_mangle]
pub unsafe extern "stdcall" fn test_long(obthis:ObVm,nargs:u32)->u32{
    let arg1 = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr()).as_ref().unwrap().get_long_unchecked();
    let arg2 = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr()).as_ref().unwrap().get_long_unchecked();

    let data = ObData::new(arg1 + arg2, ValueType::Long);
    let _ = OT_SET_RETURN_VAL(obthis.as_ptr(),&data);

    return 1;
    /* 
    第二种方式
    let mut isnull = false;
    let arg1 = OT_GET_VALPTR_ARG(obthis.as_ptr(),&mut isnull) ;
    let arg2 = OT_GET_VALPTR_ARG(obthis.as_ptr(),&mut isnull) ;
    let var1 = arg1 as i32;
    let var2 = arg2 as i32;
    let var3 = var1 + var2; 

    let mut data = ObData::new(var3, ValueType::Long);

    //let mut data = ObData::new(true, ValueType::Boolean);
    */
       
}
#[no_mangle]
pub unsafe extern "stdcall" fn test_int(obthis:ObVm,nargs:u32)->u32{
    
    let arg1 = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    let arg2 = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    
    let var1 = arg1.as_ref().unwrap().get_int_unchecked();
    let var2 = arg2.as_ref().unwrap().get_int_unchecked();

    let data = ObData::new(var1 + var2,ValueType::Int);
    let _ = OT_SET_RETURN_VAL(obthis.as_ptr(),&data);

    return 1;
 
    
}

/*
标准类型参数获取
*/
#[no_mangle]
pub unsafe extern "stdcall" fn test_type(obthis:ObVm,nargs:u32)->u32{
    let mut isnull = false;
    let v_bool      = obthis.get_next_arg().unwrap();
    let v_double    = obthis.get_next_arg().unwrap();
    let v_decimal   = obthis.get_next_arg().unwrap();
    let v_string    = obthis.get_next_arg().unwrap();
    let v_any       = obthis.get_next_arg().unwrap();
    let v_blob      = obthis.get_next_arg().unwrap();
    let v_date      = obthis.get_next_arg().unwrap();
    let v_time      = obthis.get_next_arg().unwrap();
    let v_datetime  = obthis.get_next_arg().unwrap();
    let v_longlong  = obthis.get_next_arg().unwrap();
    
    /* bool */
    let pbboolean = v_bool.get_bool_unchecked();
    /* double */
    let pbdouble = v_double.get_double_unchecked();
    /* decimal */
    let rdec = v_decimal.get_decimal_unchecked();
    let pbdec = v_decimal.get_pbdec_unchecked();
    /* string */
    let pbstr = v_string.get_string_unchecked();
    let str = pbstr.to_string_lossy();
    /* any */
    let vtype = v_any.get_type();
    /* blob */
    let rblob = v_blob.get_blob_unchecked();
    let pbblob = v_blob.get_pbblob_unchecked();
    /* date */
    let pbdate = v_date.get_pbdate_unchecked();
    /* time */
    let pbtime = v_time.get_pbdate_unchecked();
    /* datetime */
    let pbdatetime = v_datetime.get_pbdate_unchecked();
    /* longlong */
    let pblonglong = v_longlong.get_longlong_unchecked();


    let data = ObData::new(123,ValueType::Int);
    let _ = OT_SET_RETURN_VAL(obthis.as_ptr(),&data);

    return 1;
      
}

/*
标准对象获取测试
*/

#[no_mangle]
pub unsafe extern "stdcall" fn test_object(obthis:ObVm,nargs:u32)->u32{
    let v_dw1 = obthis.get_next_arg().unwrap();
    let v_dw2 = obthis.get_next_arg().unwrap();


    let pdw1 = v_dw1.get_object_unchecked();
    let pdw2 = v_dw2.get_object_unchecked();






    let _ =   obthis.set_return_long(25);
    return 1;
}
