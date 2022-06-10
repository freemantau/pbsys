use crate::pbsys::*;

pub type Pvoid = *mut std::ffi::c_void;
pub type Ot_Fieldupdate_Func = fn(ObVm,Pvoid,u32,u32)->i32;
#[repr(C)]
pub struct Ot_Ref_Pak{
    style:Ot_RefPak_Style,
    group_hndl:u16,
    ty:u16,
    flags:u16,
    refunion:Ot_Ref_Tag_Union
}
impl Ot_Ref_Pak {
    pub fn get_type(&self)->Ot_RefPak_Style{
        self.style
    }
    pub fn get_simple_ref(&self)->Option<&mut ObData>{
        if self.style == Ot_RefPak_Style::OT_SIMPLE_REF
        {
            Some(unsafe{&mut *(usize::from_le_bytes(self.refunion.unionval) as *mut ObData)})
        }
        else {
            None
        }
    }
    pub fn get_field_ref(&self){
        todo!()
    }
    pub fn get_field_item_ref(&self){
        todo!()
    }
}
#[repr(C)]
pub struct  Ot_Ref_Tag_Union{
    unionval:[u8;4]
}

#[repr(C)]
pub struct Ot_Ref_Pak_Field_Ref_Tag{
    obinst:Pvoid,
    filed_id:u16,
    field_update_func:Ot_Fieldupdate_Func,
    item_index:u32
}

#[repr(C)]
pub struct Ot_Ref_Pak_Simple_Ref_Tag{
    lvalue:ObData
}

#[derive(Clone, Copy,PartialEq, Eq)]
#[repr(u32)]
pub enum Ot_RefPak_Style {
    OT_SIMPLE_REF = 0,
    OT_FIELD_REF,
    OT_FIELD_ITEM_REF
}