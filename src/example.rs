use std::{ffi::c_void, ptr::NonNull, sync::Arc, vec};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::{arr::*, refv::*, pbsys::*,dll::*, obj::*, capi::pbtypes::{OB_THIS, POB_THIS, OB_INST_ID}, _CAPI};

#[no_mangle]
pub unsafe extern "stdcall" fn bit_or(obthis: ObVm, nargs: u32) -> u32 {
    let arg1 = obthis.get_next_arg().unwrap().get_long_unchecked();
    let arg2 = obthis.get_next_arg().unwrap().get_long_unchecked();
    let _ = obthis.set_return_long(arg1 | arg2);
    return 1;
}

#[no_mangle]
pub unsafe extern "stdcall" fn test_long(obthis: ObVm, nargs: u32) -> u32 {
    let arg1 = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr())
        .as_ref()
        .unwrap()
        .get_long_unchecked();
    let arg2 = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr())
        .as_ref()
        .unwrap()
        .get_long_unchecked();

    let data = ObData::new(arg1 + arg2, ValueType::Long);
    let _ = OT_SET_RETURN_VAL(obthis.as_ptr(), &data);

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
pub unsafe extern "stdcall" fn test_int(obthis: ObVm, nargs: u32) -> u32 {
    let arg1 = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());
    let arg2 = OT_GET_NEXT_EVALED_ARG_NO_CONVERT(obthis.as_ptr());

    let var1 = arg1.as_ref().unwrap().get_int_unchecked();
    let var2 = arg2.as_ref().unwrap().get_int_unchecked();

    let data = ObData::new(var1 + var2, ValueType::Int);
    let _ = OT_SET_RETURN_VAL(obthis.as_ptr(), &data);

    return 1;
}

/*
标准类型参数获取
*/
#[no_mangle]
pub unsafe extern "stdcall" fn test_type(obthis: ObVm, nargs: u32) -> u32 {
    let mut isnull = false;
    let v_bool = obthis.get_next_arg().unwrap();
    let v_double = obthis.get_next_arg().unwrap();
    let v_decimal = obthis.get_next_arg().unwrap();
    let v_string = obthis.get_next_arg().unwrap();
    let v_any = obthis.get_next_arg().unwrap();
    let v_blob = obthis.get_next_arg().unwrap();
    let v_date = obthis.get_next_arg().unwrap();
    let v_time = obthis.get_next_arg().unwrap();
    let v_datetime = obthis.get_next_arg().unwrap();
    let v_longlong = obthis.get_next_arg().unwrap();

    let v_non = obthis.get_next_arg();

    /* bool */
    let pbboolean = v_bool.get_bool_unchecked();
    /* double */
    let pbdouble = v_double.get_double_unchecked();
    /* decimal */
    let rdec = v_decimal.get_decimal_unchecked();
    let pbdec = v_decimal.get_pbdec_unchecked();
    /* string */
    let pbstr = v_string.get_string_unchecked();
    //let str = pbstr.to_string_lossy();
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

    let data = ObData::new(123, ValueType::Int);
    let _ = OT_SET_RETURN_VAL(obthis.as_ptr(), &data);

    return 1;
}
/*
标准对象获取测试
*/
#[no_mangle]
pub unsafe extern "stdcall" fn test_object(obthis: ObVm, nargs: u32) -> u32 {
    let v_dw1 = obthis.get_next_arg().unwrap();
    let v_dw2 = obthis.get_next_arg().unwrap();

    let pdw1 = v_dw1.get_object_unchecked();
    let pdw2 = v_dw2.get_object_unchecked();

    let _ = obthis.set_return_long(25);
    return 1;
}

#[no_mangle]
extern "stdcall" fn test_return_val(obthis: ObVm, nargs: u32) -> u32 {
    let arg1 = obthis.get_next_arg().unwrap();
    let arg2 = obthis.get_next_arg().unwrap();

    let data = match arg1.get_type() {
        ValueType::Int => ObData::new(
            arg1.get_int_unchecked() + arg2.get_int_unchecked(),
            ValueType::Int,
        ),
        ValueType::Long => ObData::new(
            arg1.get_long_unchecked() + arg2.get_long_unchecked(),
            ValueType::Long,
        ),
        ValueType::Real => {
            let myreal = arg1.get_real_unchecked() + arg2.get_real_unchecked();
            ObData::new(myreal, ValueType::Real)
        }
        ValueType::Double => {
            let v_d1 = arg1.get_double_unchecked();
            let v_d2 = arg2.get_double_unchecked();
            let v_d3 = v_d1 + v_d2;
            ObData::new_ptrvalue(&obthis, &v_d3, ValueType::Double)
            //ObData::new(arg1.get_double_unchecked() + arg2.get_double_unchecked(),ValueType::Double)
        }
        ValueType::Decimal => {
            let dec1 = arg1.get_decimal_unchecked();
            let dec2 = arg2.get_decimal_unchecked();
            let dec3 = Psh_Dec::from(dec1);
            let dec4 = Psh_Dec::from(dec1);
            ObData::new_ptrvalue(&obthis, &dec3, ValueType::Decimal)
        }
        ValueType::String => {
            let pbs1 = arg1.get_string_unchecked();
            let mut v: Vec<u16> = vec![0];
            v.extend_from_slice(pbs1.as_slice());
            let s1 = arg1.get_string_unchecked().to_string_lossy();
            let s2 = arg2.get_string_unchecked().to_string_lossy();
            let mut s3 = String::default();
            s3.push_str(&s1);
            s3.push_str(&s2);
            ObData::new_ptrvalue(&obthis, s3.as_str(), ValueType::String)
        }
        ValueType::Boolean => {
            let b1 = arg1.get_bool_unchecked();
            let b2 = arg2.get_bool_unchecked();
            ObData::new(b1 && b2, ValueType::Boolean)
        }
        ValueType::Any => todo!(),
        ValueType::Uint => ObData::new(
            arg1.get_uint_unchecked() + arg2.get_uint_unchecked(),
            ValueType::Uint,
        ),
        ValueType::Ulong => ObData::new(
            arg1.get_ulong_unchecked() + arg2.get_ulong_unchecked(),
            ValueType::Ulong,
        ),
        ValueType::Blob => {
            let b1 = arg1.get_blob_unchecked();
            let b2 = arg2.get_blob_unchecked();
            let mut b3: Vec<u8> = vec![];
            b3.extend_from_slice(&b1[0..]);
            b3.extend_from_slice(&b2[0..]);
            ObData::new_ptrvalue(&obthis, &b3[0..], ValueType::Blob)
        }
        ValueType::Date => {
            let dt1 = arg1.get_pbdate_unchecked();
            let dt2 = arg2.get_pbdate_unchecked();
            let dt3 = Psh_Time::from((NaiveDate::from_ymd(2019, 1, 5)));
            ObData::new_ptrvalue(&obthis, &dt3, ValueType::Date)
        }
        ValueType::Time => {
            let t1 = arg1.get_pbdate_unchecked();
            let t2 = arg2.get_pbdate_unchecked();
            let t3 = Psh_Time::from((NaiveTime::from_hms_milli(12, 25, 55, 123)));
            ObData::new_ptrvalue(&obthis, &t3, ValueType::Date)
        }
        ValueType::DateTime => {
            let nd = NaiveDate::from_ymd(2019, 1, 5);
            let nt = NaiveTime::from_hms_milli(12, 25, 55, 123);
            let ndt = NaiveDateTime::new(nd, nt);
            let dt = Psh_Time::from(ndt);
            ObData::new_ptrvalue(&obthis, &dt, ValueType::DateTime)
        }
        ValueType::Char => {
            let c1 = arg1.get_char_unchecked();
            let c2 = arg2.get_char_unchecked();
            ObData::new(c1 + 1, ValueType::Char)
        }
        ValueType::LongLong => {
            let var1 = arg1.get_longlong_unchecked();
            let var2 = arg2.get_longlong_unchecked();
            ObData::new_ptrvalue(&obthis, &(var1 + var2), ValueType::LongLong)
        }
        ValueType::Byte => {
            let by1 = arg1.get_byte_unchecked();
            let by2 = arg2.get_byte_unchecked();
            ObData::new(by1 + 1, ValueType::Byte)
        }
        _ => ObData::new(0, ValueType::Int),
    };
    obthis.set_return_val(&data);
    return 1;
}

#[no_mangle]
extern "stdcall" fn test_ref_val(obthis: ObVm, nargs: u32) -> u32 {
    let mut hnd = 0u32;

    let arg1 = obthis.get_next_lvalue_arg(&mut hnd).unwrap();
    let arg2 = obthis.get_next_arg().unwrap();

    match arg1.get_type() {
        ValueType::Long => {
            let v2 = arg2.get_long_unchecked();
            let refpak = arg1.get_refpak_unchecked();
            let refptr = refpak.as_ptr();
            let mut pv1 = refpak.get_simple_ref().unwrap();
            let v1 = pv1.get_long_unchecked();
            let _ = pv1.set_data_value(&(v1 + v2));
            //           obthis.assign_ref_long(refpak, 333, false);
        }

        ValueType::String => {
            let v2 = arg2.get_string_unchecked().to_string_lossy();
            let refpak = arg1.get_refpak_unchecked();
            let mut pv1 = refpak.get_simple_ref().unwrap();
            let v1 = pv1.get_string_unchecked().to_string_lossy();
            let mut v3 = String::from("nihao");
            v3.push_str(&v1);
            v3.push_str(&v2);
            pv1.set_data_ptrvalue(&obthis, &v3.as_str());
        }

        _ => {}
    }

    let data = ObData::new(true, ValueType::Boolean);
    obthis.set_return_val(&data);
    return 1;
}

#[no_mangle]
extern "stdcall" fn test_array(obthis: ObVm, nargs: u32) -> u32 {
    let arg1 = obthis.get_next_arg().unwrap();
    let arg2 = obthis.get_next_arg().unwrap();

    let parr1 = arg1.get_arrayid_unchecked();
    let parr2 = arg2.get_arrayid_unchecked();
    let arrcnt = obthis.get_array_num_items(parr1);

    let data = match arg1.get_type() {
        ValueType::Long | ValueType::Int => {
            let mut total = 0i32;
            for i in 0..arrcnt {
                let item = obthis.get_array_index(parr1, i);
                total += item.get_long_unchecked();
            }
            ObData::new(total, ValueType::Long)
        }
        ValueType::String => {
            let mut stotal = String::default();
            for i in 0..arrcnt {
                let item = obthis.get_array_index(parr1, i);
                stotal.push_str(item.get_string_unchecked().to_string_lossy().as_str());
            }
            ObData::new_ptrvalue(&obthis, stotal.as_str(), ValueType::String)
        }
        _ => ObData::new(0i32, ValueType::Long),
    };

    obthis.set_return_val(&data);
    return 1;
}

#[no_mangle]
extern "stdcall" fn test_array_ref(obthis: ObVm, nargs: u32) -> u32 {
    let mut hnd = 0u32;
    //引用类型
    let arg1 = obthis.get_next_lvalue_arg(&mut hnd).unwrap();
    //值
    let arg2 = obthis.get_next_arg().unwrap();

    //数组指针
    let parr1 = arg1.get_arrayid_unchecked();
    let parr2 = arg2.get_arrayid_unchecked();

    //此处  classid为 valuetype，OB_CLASS_ID,既 OBDATA的type
    //group_hndl 暂不明确
    let classhndl = Ob_Class_Hndl {
        group_hndl: 0xffff,
        class_id: ValueType::Long.into(),
    };
    let rtarr = obthis.array_create_unbounded(classhndl, 0);

    //数组2 长度
    let arrcnt = obthis.get_array_num_items(parr2);
    let mut j = 0u32;
    for i in (0..arrcnt).rev() {
        let d = obthis.get_array_index_mut(rtarr, j);
        //获取数组值
        let item = obthis.get_array_index(parr2, i).get_long_unchecked();
        d.set_data_value(&item);
        j += 1;
    }

    let refd = arg1.get_refpak_unchecked().get_simple_ref().unwrap();
    let _ = refd.set_data_ptrvalue(&obthis, &rtarr);

    //arg1.set_data_ptrvalue(obthis, val)

    return 1;
}



#[no_mangle]
extern "stdcall" fn testobj_create(obthis:ObVm,nargs:u32)->u32{
    let _ = obthis.no_return_val();
    return 1;
}
#[no_mangle]
extern "stdcall" fn testobj_destroy(obthis:ObVm,nargs:u32)->u32{
    let mut nullval = true;
    let obj = ObClass::default();
    let _ = obthis.get_curr_obinst(&obj, &mut nullval);

    let mut data = ObData::default();
    obthis.get_obinst_field(&obj, 2, &mut data);
    unsafe{OB_FREE_VALUE(obthis.as_ptr(),data.get_valptr::<c_void>() as Pvoid)};
    let _ = obthis.no_return_val();
    return 1;
}

#[no_mangle]
extern "stdcall" fn testobj_get(obthis:ObVm,nargs:u32)->u32{
    let mut isnull = true;
    let mut obj = ObClass::default();
    obthis.get_curr_obinst(&obj, &mut isnull);
    let mut data = ObData::default();
    obthis.get_obinst_field(&obj, 1, &mut data);
    
    obthis.set_return_val(&data);
    return 1;
}

#[no_mangle]
extern "stdcall" fn testobj_set(obthis:ObVm,nargs:u32)->u32{
    let mut isnull  = true;
    let mut obj = ObClass::default();
    let _ = obthis.get_curr_obinst(&obj, &mut isnull);

    let arg1 = obthis.get_next_arg().unwrap().get_long_unchecked();
    let mut data = ObData::new(arg1, ValueType::Long);

    obthis.set_obinst_field(&obj, 1, &mut data);

    obthis.no_return_val();
    return 1;

}

#[no_mangle]
extern "stdcall" fn append(obthis:ObVm,nargs:u32)->u32{
    let mut hnd = 0u32;
    let refd = obthis.get_next_lvalue_arg(&mut hnd).unwrap();
    let dst = refd.get_refpak_unchecked().get_simple_ref().unwrap();

    let dststr = dst.get_string_unchecked();
    let slidst = dststr.as_slice_with_nul();
    let mut v:Vec<u16> = vec![];
    v.extend_from_slice(dststr.as_slice());
    let mut tlen = dststr.len();
    for n in 0..(nargs-1)
    {
        let arg = obthis.get_next_arg().unwrap().get_string_unchecked();
        tlen += arg.len();
        v.extend_from_slice(
            if n==(nargs-2) {
                arg.as_slice_with_nul()
            }else{   
                arg.as_slice()}
        );
    }

    let t = v.as_slice();

  //  let tdup = unsafe{OB_DUP_STRING(obthis.as_ptr(),t)};

    let tall = unsafe{OB_ALLOC_STRING(obthis.as_ptr(),tlen as u32)};

    
    unsafe{std::ptr::copy_nonoverlapping(v.as_ptr(), tall.as_mut_ptr(), tlen+1)};
    
    unsafe{OB_FREE_VALUE(obthis.as_ptr(),dst.get_valptr::<c_void>() as Pvoid)}

    dst.set_data_ptr(tall.as_ptr() as usize);
    obthis.set_return_val(&ObData::new(true, ValueType::Boolean));
    return 1;
}


#[no_mangle]
extern "stdcall" fn testobj_setstring(obthis:ObVm,nargs:u32)->u32{
    let mut arg1 = obthis.get_next_arg().unwrap();

    let mut isnull = true;
    let obj = ObClass::default();
    let _ = obthis.get_curr_obinst(&obj, &mut isnull);

    let mystr = arg1.get_string_unchecked().to_string_lossy();
    let mut data = ObData::new_ptrvalue(&obthis,mystr.as_str(),ValueType::String);

    //let mut data = ObData::new_ptrvalue(&obthis, arg1.get_string_unchecked().to_string_lossy().as_str(), ValueType::String);

    obthis.set_obinst_field(&obj, 2, &mut data);

    //let ptr = arg1.val_as_ptr();
    
    let a = 100;
    let b = 200;
    //let sz = unsafe{PBSTG_SZ(obthis.as_ptr(),arg1.val_as_ptr() )};


    //let buffsize = sz;

    obthis.no_return_val();
    return 1;
}
#[no_mangle]
extern "stdcall" fn testobj_getstring(obthis:ObVm,nargs:u32)->u32{
    let mut isnull = true;
    let obj = ObClass::default();
    let _ = obthis.get_curr_obinst(&obj, &mut isnull);
    let mut data = ObData::default();
    let _ = obthis.get_obinst_field(&obj, 2, &mut data);

    let mystr = data.get_string_unchecked().to_string_lossy();

    obthis.set_return_val(&data);

    return 1;
}



#[no_mangle]
extern "stdcall" fn bit_and(obthis:POB_THIS,nargs:u32)->u32{

    let arg1 = _CAPI.vm.ot_get_next_evaled_arg_no_convert()(obthis);

    let arg2 = _CAPI.vm.ot_get_next_evaled_arg_no_convert()(obthis);

    



    let x = 100;
    let y = 200;


    _CAPI.vm.ot_no_return_val()(obthis);
    return 1;
}