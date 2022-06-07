use rust_decimal::Decimal;
use widestring::WideCString;

use crate::dll::*;
use std::ptr::NonNull;


pub type PBString = WideCString;


#[repr(C)]
pub struct _POBVM([u8;0]);
pub type  pobvm= NonNull<_POBVM>;

#[repr(transparent)]
pub struct ObVm
{
    ptr:pobvm
}
impl ObVm
{
    pub(crate) fn as_ptr(&self) ->pobvm
    {
        self.ptr
    }

    pub fn get_next_arg(&self) -> Option<&ObData>
    {
        unsafe{OT_GET_NEXT_EVALED_ARG_NO_CONVERT(self.as_ptr()).as_ref()}
    }
    pub fn set_return_val(&self,data:&ObData)
    {
        unsafe{OT_SET_RETURN_VAL(self.as_ptr(),data)};
    }
    pub fn set_return_long(&self,val:i32)
    {
        let data = ObData::new(val, ValueType::Long);
        unsafe{OT_SET_RETURN_VAL(self.as_ptr(),&data)};
    }
}


pub type ObInfo = u16;
#[repr(C,packed(1))]
pub struct ObData
{
    val:UnionValue,
    info:ObInfo,
    r#type:ValueType
}

#[repr(C)]
pub struct UnionValue
{
    data:[u8;4]
}

pub trait AsValue
{
    fn asvalue(&self)->UnionValue;
}

impl AsValue for i32
{
    fn asvalue(&self)->UnionValue {
        UnionValue{
            data:i32::to_le_bytes(*self)
        }
    }
}

impl AsValue for u32
{
    fn asvalue(&self)->UnionValue {
        UnionValue{
            data:u32::to_le_bytes(*self)
        }
    }
}

impl AsValue for i16{
    fn asvalue(&self)->UnionValue {
        UnionValue{
            data:i32::to_le_bytes((*self).into())
        }
    }
}
impl AsValue for u16{
    fn asvalue(&self)->UnionValue {
        UnionValue{
            data:u32::to_le_bytes((*self).into())
        }
    }
}
impl AsValue for f32{
    fn asvalue(&self)->UnionValue {
        UnionValue { data: f32::to_be_bytes(*self) }
    }
}


impl AsValue for bool{
    fn asvalue(&self)->UnionValue {
        UnionValue { data: [*self as u8,0,0,0] }
    }
}

impl ObData
{
    pub fn new(value:impl AsValue,valtype:ValueType)->Self
    {
        ObData{
            val:value.asvalue(),
            info:valtype.into_obinfo_value(),
            r#type:valtype,
        }
    }
    pub fn get_valptr<T>(&self) -> *const T
    {
        usize::from_le_bytes(self.val.data) as *const T
    }
    pub fn get_type(&self)->ValueType
    {
        self.r#type.into()
    }
    pub fn get_long_unchecked(&self)->i32{
        i32::from_le_bytes(self.val.data)
    }
    pub fn get_ulong_unchecked(&self)->u32{
        u32::from_le_bytes(self.val.data)
    }
    pub fn get_int_unchecked(&self)->i16{
        i16::from_le_bytes([self.val.data[0],self.val.data[1]])
    }
    pub fn get_uint_unchecked(&self)->u16{
        u16::from_le_bytes([self.val.data[0],self.val.data[1]])
    }
    pub fn get_real_unchecked(&self)->f32{
        f32::from_le_bytes(self.val.data)
    }
    pub fn get_bool_unchecked(&self)->bool{
        self.val.data[0] == 1
    }
    pub fn get_string_unchecked(&self)->PBString
    { 
        unsafe{PBString::from_ptr_str(self.get_valptr::<u16>())}
    }
     pub fn get_double_unchecked(&self)->f64
    {
        unsafe{*(self.get_valptr::<f64>())}
    }
    pub fn get_decimal_unchecked(&self)->Decimal{
        let psh_dec = unsafe{&*self.get_valptr::<Psh_Dec>()};
        psh_dec.into()
    }


}


impl ValueType
{
    pub fn into_obinfo_value(self)->ObInfo{
        match self{
            ValueType::NoType => {todo!()},
            ValueType::Int 
                |ValueType::Uint
                |ValueType::Boolean
                |ValueType::Char
                |ValueType::Byte => 0x05C0,
            ValueType::Long
                |ValueType::Ulong => 0x1DC0,
            ValueType::Real => 0x09C0,
            ValueType::Double => todo!(),
            ValueType::Decimal => todo!(),
            ValueType::String => 0x0DC0,
            ValueType::Any => todo!(),
            ValueType::Blob => todo!(),
            ValueType::Date => todo!(),
            ValueType::Time => todo!(),
            ValueType::DateTime => todo!(),
            ValueType::Dummy1 => todo!(),
            ValueType::Dummy2 => todo!(),
            ValueType::Dummy3 => todo!(),
            ValueType::Dummy4 => todo!(),
            ValueType::LongLong => todo!(),
        }
    }
    pub fn into_obinfo_readonly(self)->ObInfo{
        todo!()
    }    
    pub fn into_obinfo_ref(self)->ObInfo{
        todo!()
    }
    pub fn into_obinfo_null(self)->ObInfo{
        todo!()
    }
}


#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueType {
    NoType = 0,
    Int,
    Long,
    Real,
    Double,
    Decimal,
    String,
    Boolean,
    Any,
    Uint,
    Ulong,
    Blob,
    Date,
    Time,
    DateTime,
    Dummy1,
    Dummy2,
    Dummy3,
    Char,
    Dummy4,
    LongLong,
    Byte
}

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ob_Data_Style
{
    UNDECLARED_STYLE = 0,
    INT_STYLE = 1,
    FLOAT_STYLE = 2,
    PTR_STYLE = 3,
    CONST_STYLE = 4,
    ID_STYLE = 5,
    OBINST_STYLE = 6,
    LONG_STYLE = 7
}

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FieldType
{
    OB_TYPEDEF_FIELD = 0,                    // For instance variables
    OB_INSTVAR_FIELD = 0x0500
}
#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValAccess
{
    OB_GLOBAL_VAR = 0,                    // For globally scoped variables
    OB_SHARED_VAR = 1
}

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ob_Group_Types
{
    OB_SIMPLE = 0,
    OB_ARRAY

}


#[repr(C)]
pub struct Psh_Dec
{
    v:[u16;7],
    flags:u16
}
/* flags 1：小数位,2：正负 */
/* v:u128 */
impl From<&Psh_Dec> for Decimal
{
    fn from(pdec: &Psh_Dec) -> Self {
        let num = u128::from(pdec.v[0]) |
                        (16<<u128::from(pdec.v[1]))|
                        (32<<u128::from(pdec.v[2]))|
                        (48<<u128::from(pdec.v[3]))|
                        (64<<u128::from(pdec.v[4]))|
                        (80<<u128::from(pdec.v[5]))|
                        (96<<u128::from(pdec.v[6]));
        let scale = pdec.flags.to_be_bytes()[0];
        let isnag = pdec.flags.to_be_bytes()[1];
        
        let rt:i128 = match isnag
        {
            1 => (0 - num).try_into().unwrap(),
            _ => num.try_into().unwrap()
        };
        Decimal::from_i128_with_scale(rt, scale.into())
    }
}
/*
#define ob_set_data_info(node,style,typ,group,vartype)			   	\
	((node)->info = (OB_INFO_FLAGS) (							   	\
					 (OB_PUBLIC_MEMBER << DATA_ACCESS_SHIFT)	|  	\
					 ((group) << DATA_GROUP_SHIFT)				|  	\
					 (0 << DATA_FIELDTYPE_SHIFT)				|  	\
					 ((style) << DATA_STYLE_SHIFT)				|  	\
					 (USED << DATA_STATUS_SHIFT)				|  	\
					 (OB_DIRECT_REF << DATA_REFTYPE_SHIFT) 		|  	\
					 (0 << DATA_TYPEARGS_SHIFT)),					\
	 (node)->type = (OB_CLASS_ID)typ								\
	)
*/