use ::core::time::Duration;
use ::std::ffi::c_void;
use std::ptr;

use common_libs::board_memory_data::SharedStateReader;
use vrot_pe_lib::VrotPE;

#[repr(C)]
pub struct TimerObj
{
    vrot_pe: *mut VrotPE,
    interval: Duration,
}

pub type IRQTrigger = unsafe extern "C" fn();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pe_init(
    timer: *mut TimerObj,
    vrot: *mut vrot_pe_lib::VrotPE,
    arg: *mut c_void,
) -> i32
{
    assert!(!timer.is_null());

    let timer = unsafe { timer.as_mut() }.unwrap();
    let interval = Duration::from_millis(1000);

    unsafe {
        ptr::write(
            timer,
            TimerObj {
                vrot_pe: vrot,
                interval,
            },
        );
    }

    println!("rtc_timer: init done");
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pe_main(timer: *mut TimerObj, _board_memory_data: SharedStateReader)
{
    assert!(!timer.is_null());
    let timer = unsafe { timer.as_mut() }.unwrap();

    println!("rtc timer: main started");

    loop
    {
        ::std::thread::sleep(timer.interval);

        println!("rtc timer: triggering");
        unsafe { VrotPE::pe_vrot_trigger_hw_interrupt(timer.vrot_pe, 0) };
    }
}
