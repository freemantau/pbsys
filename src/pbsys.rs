use crate::{
    arr::*,
    dll::*,
    refv::*,
    obj::*
};
use chrono::{
    naive::{NaiveDate, NaiveDateTime, NaiveTime},
    Datelike, Timelike,
};
use core::num;
use num_enum::TryFromPrimitive;
use rust_decimal::Decimal;
use std::ptr::{null, NonNull};
use widestring::WideCString;

pub type PBString = WideCString;

pub type Pob_Data = Option<*mut ObData>;

#[repr(C)]
pub struct _POBVM([u8; 0]);
pub type pobvm = NonNull<_POBVM>;

#[repr(transparent)]
pub struct ObVm {
    ptr: pobvm,
}
impl ObVm {
    pub(crate) fn as_ptr(&self) -> pobvm {
        self.ptr
    }
    ///
    /// 获取value指类型参数
    ///
    pub fn get_next_arg(&self) -> Option<&ObData> {
        unsafe { OT_GET_NEXT_EVALED_ARG_NO_CONVERT(self.as_ptr()).as_ref() }
    }
    ///
    /// 获取ref引用类型参数
    ///
    pub fn get_next_lvalue_arg(&self, hnd: &mut u32) -> Option<&mut ObData> {
        unsafe { OT_GET_NEXT_LVALUE_ARG(self.as_ptr(), hnd).as_mut() }
    }
    ///
    /// 获取当前调用实例
    /// 
    pub fn get_curr_obinst(&self,pobinst:&ObClass,nullval:&mut bool){
        unsafe { OT_GET_CURR_OBINST_EXPR(self.as_ptr(), pobinst,nullval) };
    }
    ///
    /// 
    /// 
    pub fn get_obinst_field(&self,pobinst:&ObClass,field_id:u32,data:&mut ObData){
        let v = unsafe {OB_GET_FIELD(self.as_ptr(),pobinst.as_ptr(),field_id,data)};
    }
    ///
    /// 
    /// 
    pub fn set_obinst_field(&self,pobinst:&ObClass,field_id:u32,data:&mut ObData){
        unsafe { OB_SET_FIELD(self.as_ptr(),pobinst.as_ptr(),field_id,data)}
    }
}
impl ObVm {
    pub fn no_return_val(&self) {
        unsafe { OT_NO_RETURN_VAL(self.as_ptr()) };
    }
    pub fn set_return_val(&self, data: &ObData) {
        unsafe { OT_SET_RETURN_VAL(self.as_ptr(), data) };
    }
    pub fn set_return_long(&self, val: i32) {
        let data = ObData::new(val, ValueType::Long);
        unsafe { OT_SET_RETURN_VAL(self.as_ptr(), &data) };
    }
}
impl ObVm {
    pub fn dup_string<'a>(&self, val: &'a [u16]) -> &'a [u16] {
        unsafe { OB_DUP_STRING(self.as_ptr(), val) }
    }
    pub fn dup_blob<'a>(&self, val: &'a Psh_Binary) -> &'a Psh_Binary {
        unsafe { OB_DUP_BLOB(self.as_ptr(), val) }
    }
    pub fn dup_dec<'a>(&self, val: &'a Psh_Dec) -> &'a Psh_Dec {
        unsafe { OB_DUP_DEC(self.as_ptr(), val) }
    }
    pub fn dup_double<'a>(&self, val: &'a f64) -> &'a f64 {
        unsafe { OB_DUP_DOUBLE(self.as_ptr(), val) }
    }
    pub fn dup_longlong<'a>(&self, val: &'a i64) -> &'a i64 {
        unsafe { OB_DUP_LONGLONG(self.as_ptr(), val) }
    }
    pub fn dup_time<'a>(&self, val: &'a Psh_Time) -> &'a Psh_Time {
        unsafe { OB_DUP_TIME(self.as_ptr(), val) }
    }
}
impl ObVm {
    ///
    /// 获取数组长度
    ///
    pub fn get_array_num_items(&self, ptr: Parray) -> u32 {
        unsafe { OT_ARRAY_NUM_ITEMS(self.as_ptr(), ptr) }
    }
    ///
    /// 获取数组数据引用
    ///
    pub fn get_array_index(&self, ptr: Parray, index: u32) -> &ObData {
        unsafe { &*(OT_ARRAY_INDEX(self.as_ptr(), ptr, index)) }
    }
    ///
    /// 获取数组数据引用mut
    ///
    pub fn get_array_index_mut(&self, ptr: Parray, index: u32) -> &mut ObData {
        unsafe { &mut *(OT_ARRAY_INDEX(self.as_ptr(), ptr, index)) }
    }
    ///
    /// create boundarray
    ///
    pub fn array_create_bounded(
        &self,
        num_items: u32,
        elmttype: Ob_Class_Hndl,
        varinfo: u16,
        dim: u16,
        boundsarray: Pvoid,
    ) -> Parray {
        unsafe {
            OT_ARRAY_CREATE_BOUNDED(
                self.as_ptr(),
                num_items,
                elmttype,
                varinfo,
                dim,
                boundsarray,
            )
        }
    }
    ///
    /// create unboundarray
    ///
    pub fn array_create_unbounded(&self, elmttype: Ob_Class_Hndl, varinfo: u16) -> Parray {
        unsafe { OT_ARRAY_CREATE_UNBOUNDED(self.as_ptr(), elmttype, varinfo) }
    }
}

///
/// refpak method begin (TODO!) 未知
///
impl ObVm {
    pub fn assign_ref_int(&self, refpak: &Ot_Ref_Pak, val: i32, nullval: bool) {
        unsafe { OT_ASSIGN_REF_INT(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_uint(&self, refpak: &Ot_Ref_Pak, val: u32, nullval: bool) {
        unsafe { OT_ASSIGN_REF_UINT(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_byte(&self, refpak: &Ot_Ref_Pak, val: u8, nullval: bool) {
        unsafe { OT_ASSIGN_REF_BYTE(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_long(&self, refpak: &Ot_Ref_Pak, val: i32, nullval: bool) {
        unsafe { OT_ASSIGN_REF_LONG(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_ulong(&self, refpak: &Ot_Ref_Pak, val: u32, nullval: bool) {
        unsafe { OT_ASSIGN_REF_ULONG(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_dec(&self, refpak: &Ot_Ref_Pak, val: &Psh_Dec, nullval: bool) {
        unsafe { OT_ASSIGN_REF_DEC(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_float(&self, refpak: &Ot_Ref_Pak, val: f32, nullval: bool) {
        unsafe { OT_ASSIGN_REF_FLOAT(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_double(&self, refpak: &Ot_Ref_Pak, val: &f64, nullval: bool) {
        unsafe { OT_ASSIGN_REF_DOUBLE(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_longlong(&self, refpak: &Ot_Ref_Pak, val: &i64, nullval: bool) {
        unsafe { OT_ASSIGN_REF_LONGLONG(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_string(&self, refpak: &Ot_Ref_Pak, val: &[u16], nullval: bool) {
        unsafe { OT_ASSIGN_REF_STRING(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_bool(&self, refpak: &Ot_Ref_Pak, val: bool, nullval: bool) {
        unsafe { OT_ASSIGN_REF_BOOL(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_char(&self, refpak: &Ot_Ref_Pak, val: u16, nullval: bool) {
        unsafe { OT_ASSIGN_REF_CHAR(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_blob(&self, refpak: &Ot_Ref_Pak, val: &Psh_Binary, nullval: bool) {
        unsafe { OT_ASSIGN_REF_BLOB(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_datetime(&self, refpak: &Ot_Ref_Pak, val: &Psh_Time, nullval: bool) {
        unsafe { OT_ASSIGN_REF_DATETIME(self.as_ptr(), refpak, val, nullval) };
    }
    pub fn assign_ref_obinst(&self, refpak: &Ot_Ref_Pak, val: Pvoid, nullval: bool, ty: u16) {
        unsafe { OT_ASSIGN_REF_OBINST(self.as_ptr(), refpak, val, nullval, ty) };
    }
    pub fn assign_ref_enum(&self, refpak: &Ot_Ref_Pak, val: i32, nullval: bool, ty: u16) {
        unsafe { OT_ASSIGN_REF_ENUM(self.as_ptr(), refpak, val, nullval, ty) };
    }
    pub fn assign_ref_array(&self, refpak: &Ot_Ref_Pak, val: Parray, nullval: bool, ty: u16) {
        unsafe { OT_ASSIGN_REF_ARRAY(self.as_ptr(), refpak, val, nullval, ty) };
    }
    pub fn assign_ref_any(&self, refpak: &Ot_Ref_Pak, val: &ObData, rhs_class_id: u16) {
        unsafe { OT_ASSIGN_REF_ANY(self.as_ptr(), refpak, val, rhs_class_id) };
    }
}






pub type ObInfo = u16;
#[repr(C, packed(1))]
pub struct ObData {
    val: UnionValue,
    info: ObInfo,
    r#type: ValueType,
}

#[derive(Default)]
#[repr(C)]
pub struct UnionValue {
    data: [u8; 4],
}

///
/// 对象实例
/// Ob_INST_ID
///
///
#[repr(C)]
pub struct _POBINSTID([u8; 0]);
pub type pobinstid = NonNull<_POBINSTID>;

#[repr(transparent)]
pub struct ObInstId {
    ptr: pobinstid,
}

pub trait AsValue {
    fn asvalue(&self) -> UnionValue;
}

impl AsValue for u8 {
    fn asvalue(&self) -> UnionValue {
        UnionValue {
            data: u32::to_le_bytes((*self).into()),
        }
    }
}
impl AsValue for i32 {
    fn asvalue(&self) -> UnionValue {
        UnionValue {
            data: i32::to_le_bytes(*self),
        }
    }
}
impl AsValue for u32 {
    fn asvalue(&self) -> UnionValue {
        UnionValue {
            data: u32::to_le_bytes(*self),
        }
    }
}
impl AsValue for i16 {
    fn asvalue(&self) -> UnionValue {
        UnionValue {
            data: i32::to_le_bytes((*self).into()),
        }
    }
}
impl AsValue for u16 {
    fn asvalue(&self) -> UnionValue {
        UnionValue {
            data: u32::to_le_bytes((*self).into()),
        }
    }
}
impl AsValue for f32 {
    fn asvalue(&self) -> UnionValue {
        UnionValue {
            data: f32::to_le_bytes(*self),
        }
    }
}
impl AsValue for bool {
    fn asvalue(&self) -> UnionValue {
        UnionValue {
            data: [*self as u8, 0, 0, 0],
        }
    }
}

pub trait AsPtrValue {
    fn asptrvalue(&self, obthis: &ObVm) -> UnionValue;
}
impl AsPtrValue for &f64 {
    fn asptrvalue(&self, obthis: &ObVm) -> UnionValue {
        UnionValue {
            data: (obthis.dup_double(self) as *const _ as usize).to_le_bytes(),
        }
    }
}
impl AsPtrValue for &str {
    fn asptrvalue(&self, obthis: &ObVm) -> UnionValue {
        let stru16 = self.encode_utf16().collect::<Vec<u16>>();
        UnionValue {
            data: (obthis.dup_string(&stru16[0..]).as_ptr() as usize).to_le_bytes(),
        }
    }
}
impl AsPtrValue for &[u16] {
    fn asptrvalue(&self, obthis: &ObVm) -> UnionValue {
        UnionValue {
            data: (obthis.dup_string(self).as_ptr() as usize).to_le_bytes(),
        }
    }
}
impl AsPtrValue for &i64 {
    fn asptrvalue(&self, obthis: &ObVm) -> UnionValue {
        UnionValue {
            data: (obthis.dup_longlong(self) as *const _ as usize).to_le_bytes(),
        }
    }
}
impl AsPtrValue for &[u8] {
    fn asptrvalue(&self, obthis: &ObVm) -> UnionValue {
        let ulen = self.len() as u32;
        let pblob = Psh_Binary {
            len: ulen,
            data: [0],
        };
        let dp = obthis.dup_blob(&pblob);
        let src = self.as_ptr();
        let dst = &dp.data as *const _ as *mut u8;
        let _ = unsafe { std::ptr::copy_nonoverlapping(src, dst, ulen as usize) };
        UnionValue {
            data: (dp as *const _ as usize).to_le_bytes(),
        }
    }
}
impl AsPtrValue for &Psh_Dec {
    fn asptrvalue(&self, obthis: &ObVm) -> UnionValue {
        UnionValue {
            data: (obthis.dup_dec(self) as *const _ as usize).to_le_bytes(),
        }
    }
}
impl AsPtrValue for &Psh_Time {
    fn asptrvalue(&self, obthis: &ObVm) -> UnionValue {
        UnionValue {
            data: (obthis.dup_time(self) as *const _ as usize).to_le_bytes(),
        }
    }
}

impl AsPtrValue for Parray {
    fn asptrvalue(&self, obthis: &ObVm) -> UnionValue {
        let p = *self as *const _ as usize;
        let p2 = p;
        UnionValue {
            data: (*self as *const _ as usize).to_le_bytes(),
        }
    }
}

impl Default for ObData {
    fn default() -> Self {
        Self {
            val: UnionValue::default(),
            info: Default::default(),
            r#type: ValueType::NoType,
        }
    }
}

impl ObData {
    pub fn new(value: impl AsValue, valtype: ValueType) -> Self {
        ObData {
            val: value.asvalue(),
            info: valtype.into_obinfo_value(),
            r#type: valtype,
        }
    }
    pub fn new_ptrvalue(obthis: &ObVm, ptrvalue: impl AsPtrValue, valtype: ValueType) -> Self {
        ObData {
            val: ptrvalue.asptrvalue(obthis),
            info: valtype.into_obinfo_value(),
            r#type: valtype,
        }
    }
}

impl ObData {
    pub fn as_ptr(&self) -> Pvoid {
        self as *const _ as Pvoid
    }
}

impl ObData {
    pub fn get_valptr<T>(&self) -> *const T {
        usize::from_le_bytes(self.val.data) as *const T
    }
    pub fn get_type(&self) -> ValueType {
        self.r#type
    }
    pub fn get_type_unchecked(&self) -> u16 {
        self.r#type.into()
    }
    pub fn get_byte_unchecked(&self) -> u8 {
        self.val.data[0]
    }
    pub fn get_char_unchecked(&self) -> u16 {
        u16::from_le_bytes([self.val.data[0], self.val.data[1]])
    }
    pub fn get_long_unchecked(&self) -> i32 {
        i32::from_le_bytes(self.val.data)
    }
    pub fn get_ulong_unchecked(&self) -> u32 {
        u32::from_le_bytes(self.val.data)
    }
    pub fn get_int_unchecked(&self) -> i16 {
        i16::from_le_bytes([self.val.data[0], self.val.data[1]])
    }
    pub fn get_uint_unchecked(&self) -> u16 {
        u16::from_le_bytes([self.val.data[0], self.val.data[1]])
    }
    pub fn get_real_unchecked(&self) -> f32 {
        f32::from_le_bytes(self.val.data)
    }
    pub fn get_bool_unchecked(&self) -> bool {
        self.val.data[0] == 1
    }
    pub fn get_string_unchecked(&self) -> PBString {
        unsafe { PBString::from_ptr_str(self.get_valptr::<u16>()) }
    }
    pub fn get_double_unchecked(&self) -> f64 {
        unsafe { *(self.get_valptr::<f64>()) }
    }
    pub fn get_decimal_unchecked(&self) -> Decimal {
        let psh_dec = unsafe { &*self.get_valptr::<Psh_Dec>() };
        psh_dec.into()
    }
    pub fn get_pbdec_unchecked(&self) -> &Psh_Dec {
        unsafe { &*self.get_valptr::<Psh_Dec>() }
    }
    pub fn get_pbblob_unchecked(&self) -> &Psh_Binary {
        unsafe { &*(self.get_valptr::<Psh_Binary>()) }
    }
    pub fn get_blob_unchecked(&self) -> Vec<u8> {
        self.get_pbblob_unchecked().into()
    }
    pub fn get_pbdate_unchecked(&self) -> &Psh_Time {
        unsafe { &*(self.get_valptr::<Psh_Time>()) }
    }
    pub fn get_longlong_unchecked(&self) -> i64 {
        unsafe { *(self.get_valptr::<i64>()) }
    }
    pub fn get_object_unchecked(&self) -> &ObInstId {
        unsafe { &*(self.get_valptr::<ObInstId>()) }
    }
    pub fn get_arrayid_unchecked(&self) -> Parray {
        usize::from_le_bytes(self.val.data) as Parray
    }
}

///
/// array
///
impl ObData {
    pub fn get_data_array(&self) {
        todo!()
    }
    pub fn get_data_arrayinst(&self) {
        todo!()
    }
}

impl ObData {
    pub fn get_valptr_ref<T>(&mut self) -> *mut T {
        usize::from_le_bytes(self.val.data) as *mut T
    }
    pub fn get_refpak_unchecked(&mut self) -> &mut Ot_Ref_Pak {
        unsafe { &mut *(usize::from_le_bytes(self.val.data) as *mut Ot_Ref_Pak) }
    }
}

impl ObData {
    pub fn set_data_value<T>(&mut self, val: &T)
    where
        T: AsValue,
    {
        self.val = val.asvalue();
    }
    pub fn set_data_ptrvalue<T>(&mut self, obthis: &ObVm, val: &T)
    where
        T: AsPtrValue,
    {
        self.val = val.asptrvalue(obthis)
    }
}

impl ValueType {
    pub fn into_obinfo_value(self) -> ObInfo {
        match self {
            ValueType::NoType => {
                todo!()
            }
            ValueType::Int
            | ValueType::Uint
            | ValueType::Boolean
            | ValueType::Char
            | ValueType::Byte => 0x05C0,
            ValueType::Long | ValueType::Ulong => 0x1DC0,
            ValueType::Real => 0x0900,
            ValueType::Double => 0x0D00,
            ValueType::Decimal => 0x0D00,
            ValueType::String => 0x0D00,
            ValueType::Any => todo!(),
            ValueType::Blob => 0x0D00,
            ValueType::Date => 0x0D00,
            ValueType::Time => 0x0D00,
            ValueType::DateTime => 0x0D00,
            ValueType::Dummy1 => todo!(),
            ValueType::Dummy2 => todo!(),
            ValueType::Dummy3 => todo!(),
            ValueType::Dummy4 => todo!(),
            ValueType::LongLong => 0x0D00,
        }
    }
    pub fn into_obinfo_readonly(self) -> ObInfo {
        todo!()
    }
    pub fn into_obinfo_ref(self) -> ObInfo {
        match self {
            ValueType::NoType => {
                todo!()
            }
            ValueType::Int
            | ValueType::Uint
            | ValueType::Boolean
            | ValueType::Char
            | ValueType::Byte => 0x05C0,
            ValueType::Long | ValueType::Ulong => 0x1DC0,
            ValueType::Real => 0x0980,
            ValueType::Double => 0x0D00,
            ValueType::Decimal => 0x0D00,
            ValueType::String => 0x0D00,
            ValueType::Any => todo!(),
            ValueType::Blob => 0x0D00,
            ValueType::Date => 0x0D00,
            ValueType::Time => 0x0D00,
            ValueType::DateTime => 0x0D00,
            ValueType::Dummy1 => todo!(),
            ValueType::Dummy2 => todo!(),
            ValueType::Dummy3 => todo!(),
            ValueType::Dummy4 => todo!(),
            ValueType::LongLong => 0x0D00,
        }
    }
    pub fn into_obinfo_null(self) -> ObInfo {
        todo!()
    }
}

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
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
    Byte,
}
impl Into<u16> for ValueType {
    fn into(self) -> u16 {
        self as u16
    }
}

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ob_Data_Style {
    UNDECLARED_STYLE = 0,
    INT_STYLE = 1,
    FLOAT_STYLE = 2,
    PTR_STYLE = 3,
    CONST_STYLE = 4,
    ID_STYLE = 5,
    OBINST_STYLE = 6,
    LONG_STYLE = 7,
}

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FieldType {
    OB_TYPEDEF_FIELD = 0, // For instance variables
    OB_INSTVAR_FIELD = 0x0500,
}
#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValAccess {
    OB_GLOBAL_VAR = 0, // For globally scoped variables
    OB_SHARED_VAR = 1,
}

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ob_Group_Types {
    OB_SIMPLE = 0,
    OB_ARRAY,
}

#[repr(C)]
pub struct Ob_Class_Hndl {
    pub(crate) group_hndl: u16,
    pub(crate) class_id: u16,
}

#[repr(C)]
pub struct Psh_Binary {
    len: u32,
    data: [u8; 1],
}

impl Into<Vec<u8>> for &Psh_Binary {
    fn into(self) -> Vec<u8> {
        unsafe {
            Vec::from_raw_parts(
                self.data.as_ptr() as *mut u8,
                self.len as usize,
                self.len as usize,
            )
        }
        //unsafe{Vec::from_raw_parts(self.data as *const u8 as *mut u8, self.len as usize, self.len as usize)}
    }
}

impl Psh_Binary {
    fn len(&self) -> u32 {
        self.len
    }
}

#[repr(C)]
pub struct Psh_Dec {
    v: [u16; 7],
    flags: [u8; 2],
}
/* flags 1：小数位,0：正负 */
/* v:u128 */
impl Into<Decimal> for &Psh_Dec {
    fn into(self) -> Decimal {
        let num: u128 = u128::from(self.v[0])
            | (u128::from(self.v[1]) << 16)
            | (u128::from(self.v[2]) << 32)
            | (u128::from(self.v[3]) << 48)
            | (u128::from(self.v[4]) << 64)
            | (u128::from(self.v[5]) << 80)
            | (u128::from(self.v[6]) << 96);
        let scale = self.flags[1];
        let isnag = self.flags[0];

        let rt: i128 = match isnag {
            1 => (!num + 1) as i128,
            _ => num as i128,
        };
        Decimal::from_i128_with_scale(rt, scale.into())
    }
}
impl From<Decimal> for Psh_Dec {
    fn from(dec: Decimal) -> Self {
        let isnag = dec.is_sign_negative();
        let scale = dec.scale();
        let mants = match isnag {
            true => (!dec.mantissa() + 1) as u128,
            false => dec.mantissa() as u128,
        };
        let b = mants.to_le_bytes();
        Psh_Dec {
            v: [
                u16::from_le_bytes([b[0], b[1]]),
                u16::from_le_bytes([b[2], b[3]]),
                u16::from_le_bytes([b[4], b[5]]),
                u16::from_le_bytes([b[6], b[7]]),
                u16::from_le_bytes([b[8], b[9]]),
                u16::from_le_bytes([b[10], b[11]]),
                u16::from_le_bytes([b[12], b[13]]),
            ],
            flags: [isnag.into(), scale.to_le_bytes()[0]],
        }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct Psh_Time {
    tm_msec: u32,
    tm_year: i16,
    tm_mon: u8,
    tm_day: u8,
    tm_hour: u8,
    tm_min: u8,
    tm_sec: u8,
    tm_filter: u8,
}
impl Into<NaiveDate> for &Psh_Time {
    fn into(self) -> NaiveDate {
        NaiveDate::from_ymd(
            (self.tm_year + 1900).into(),
            (self.tm_mon + 1).into(),
            self.tm_day.into(),
        )
    }
}
impl Into<NaiveTime> for &Psh_Time {
    fn into(self) -> NaiveTime {
        NaiveTime::from_hms_milli(
            self.tm_hour.into(),
            self.tm_min.into(),
            self.tm_sec.into(),
            self.tm_msec,
        )
    }
}
impl Into<NaiveDateTime> for &Psh_Time {
    fn into(self) -> NaiveDateTime {
        NaiveDateTime::new(self.into(), self.into())
    }
}

impl From<NaiveDate> for Psh_Time {
    fn from(nd: NaiveDate) -> Self {
        Psh_Time {
            tm_year: (nd.year() - 1900) as i16,
            tm_mon: (nd.month() - 1) as u8,
            tm_day: nd.day() as u8,
            ..Default::default()
        }
    }
}
impl From<NaiveTime> for Psh_Time {
    fn from(nt: NaiveTime) -> Self {
        Psh_Time {
            tm_hour: nt.hour() as u8,
            tm_min: nt.minute() as u8,
            tm_sec: nt.second() as u8,
            tm_msec: nt.nanosecond() / 1_000_000,
            ..Default::default()
        }
    }
}
impl From<NaiveDateTime> for Psh_Time {
    fn from(ndt: NaiveDateTime) -> Self {
        Psh_Time {
            tm_year: (ndt.year() - 1900) as i16,
            tm_mon: (ndt.month() - 1) as u8,
            tm_day: ndt.day() as u8,
            tm_hour: ndt.hour() as u8,
            tm_min: ndt.minute() as u8,
            tm_sec: ndt.second() as u8,
            tm_msec: ndt.nanosecond() / 1_000_000,
            ..Default::default()
        }
    }
}

#[cfg(test)]
use super::*;
#[test]
fn test_dec() {
    /* pb -0.00123451 */
    let psh_dec = Psh_Dec {
        v: [57915, 1, 0, 0, 0, 0, 0],
        flags: [1, 8],
    };
    let mydec: Decimal = (&psh_dec).into();
    assert_eq!(Decimal::from_i128_with_scale(-123451, 8), mydec);

    let dec = Decimal::from_i128_with_scale(-123451, 8);
    let pdec = Psh_Dec::from(dec);
    assert_eq!([57915, 1, 0, 0, 0, 0, 0], pdec.v);
    assert_eq!([1, 8], pdec.flags);
}
