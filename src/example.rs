use crate::*;


#[no_mangle]
pub unsafe extern "stdcall" fn bit_or(obthis:ObVm,nargs:u32)->i32{

    let arg1 = obthis.get_next_arg().unwrap().get_long_unchecked();
    let arg2 = obthis.get_next_arg().unwrap().get_long_unchecked();
    let _ = obthis.set_return_long(arg1 | arg2);
    return 1;
}



#[no_mangle]
pub unsafe extern "stdcall" fn test_long(obthis:ObVm,nargs:u32)->i32{
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
pub unsafe extern "stdcall" fn test_int(obthis:ObVm,nargs:u32)->i16{
    
    let arg1 = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    let arg2 = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    
    let var1 = arg1.as_ref().unwrap().get_int_unchecked();
    let var2 = arg2.as_ref().unwrap().get_int_unchecked();

    let data = ObData::new(var1 + var2,ValueType::Int);
    let _ = OT_SET_RETURN_VAL(obthis.as_ptr(),&data);

    return 1;
 
    
}

#[no_mangle]
pub unsafe extern "stdcall" fn test_type(obthis:ObVm,nargs:u32)->i16{
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
    let pbdec = v_decimal.get_valptr::<Psh_Dec>();
    let pbdesref = &(*pbdec);
    /* string */
    let pbstr = v_string.get_string_unchecked();
    let str = pbstr.to_string_lossy();
    /* any */
    /* blob */
    /* date */
    /* time */
    /* datetime */
    /* longlong */
    let data = ObData::new(123,ValueType::Int);
    let _ = OT_SET_RETURN_VAL(obthis.as_ptr(),&data);

    return 1;
      
}