pub mod arr;
pub mod dll;
pub mod example;
pub mod pbsys;
pub mod refv;
pub mod obj;
mod capi;


extern crate log;


use lazy_static::lazy_static;

use crate::capi::IPbAPI;

pub static PBVERSION:&'static str = "125";
lazy_static!{
    pub static ref _CAPI:IPbAPI = IPbAPI::init(PBVERSION);
}





#[no_mangle]
extern "stdcall" fn DllMain(hinstDLL: *mut u8, reason: u32, reserved: *mut u8) -> u32 {
    match reason {
        1 => {
            // ATTACH
            env_logger::init();
        }
        0 => {
            // DETACH
        }
        _ => (),
    }
    return 1;
}