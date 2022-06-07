pub mod example;
pub mod pbsys;
pub mod dll;
use pbsys::*;
use dll::*;


use std::mem;

#[macro_use]
extern crate log;


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


