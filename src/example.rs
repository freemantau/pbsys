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
    
    let v_double = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    let v_decimal = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    let v_string = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    let v_any = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    let v_blob = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    let v_date = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    let v_time = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    let v_datetime= OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    let v_longlong = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    
    //let var1 = arg1.as_ref().unwrap().get_int_unchecked();
    //let var2 = arg2.as_ref().unwrap().get_int_unchecked();

    let strptr = (*v_string).get_valptr();
    

    let data = ObData::new(123,ValueType::Int);
    let _ = OT_SET_RETURN_VAL(obthis.as_ptr(),&data);

    return 1;
      
}