use crate::dll::*;
use std::ptr::NonNull;
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


impl AsValue for bool{
    fn asvalue(&self)->UnionValue {
        todo!()
    }
}

impl ObData
{
    pub fn new(value:impl AsValue,valtype:ValueType)->Self
    {
        ObData{
            val:value.asvalue(),
            info:valtype.into(),
            r#type:valtype,
        }
    }
    pub fn get_valptr(&self) -> *const u8
    {
        usize::from_le_bytes(self.val.data) as *const u8
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




}

fn ob_get_data_info(style:Ob_Data_Style,grp:Ob_Group_Types)->ObInfo
{
    match style{
        Ob_Data_Style::UNDECLARED_STYLE => todo!(),
        Ob_Data_Style::INT_STYLE => (0x05C0),
        Ob_Data_Style::FLOAT_STYLE => (0x09C0),
        Ob_Data_Style::PTR_STYLE => (0x0DC0),
        Ob_Data_Style::CONST_STYLE => todo!(),
        Ob_Data_Style::ID_STYLE => todo!(),
        Ob_Data_Style::OBINST_STYLE => todo!(),
        Ob_Data_Style::LONG_STYLE => (0x1DC0),
    }
}

impl From<ValueType> for ObInfo
{
    fn from(typ: ValueType) -> Self {
        match typ {
            ValueType::NoType => {todo!()},
            ValueType::Int 
                |ValueType::Uint
                |ValueType::Boolean
                |ValueType::Char
                |ValueType::Byte => 
                {ob_get_data_info(Ob_Data_Style::INT_STYLE,Ob_Group_Types::OB_SIMPLE)},
            ValueType::Long
                |ValueType::Ulong => 
                {ob_get_data_info(Ob_Data_Style::LONG_STYLE,Ob_Group_Types::OB_SIMPLE)},
            ValueType::Real => 
                {ob_get_data_info(Ob_Data_Style::FLOAT_STYLE,Ob_Group_Types::OB_SIMPLE)},
            ValueType::Double => todo!(),
            ValueType::Decimal => todo!(),
            ValueType::String => {ob_get_data_info(Ob_Data_Style::PTR_STYLE,Ob_Group_Types::OB_SIMPLE)},
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