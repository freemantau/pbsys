pub mod value;
pub mod vargs;
pub mod pbtypes;


mod pbvm;
mod pbshr;
mod pbsys;
mod pbdwe;

use self::{
    pbvm::IVmAPI,
    pbshr::IShrAPI,
    pbdwe::IDweAPI,
    pbsys::ISysAPI
};

pub struct IPbAPI{
    pub vm:IVmAPI,
    pub shr:IShrAPI,
    pub dwe:IDweAPI,
    pub sys:ISysAPI
}

impl IPbAPI{
    ///
    /// 初始化
    /// 
    pub fn init(ver:&str)->Self{
        IPbAPI { 
            vm: IVmAPI::load(ver), 
            shr: IShrAPI::load(ver), 
            dwe: IDweAPI::load(ver), 
            sys: ISysAPI::load(ver), 
        }
    }

    ///
    /// 释放
    /// 
    pub fn free(){
        todo!()
    }
}

