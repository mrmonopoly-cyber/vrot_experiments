use ::core::ptr;

use ::std::ffi;
use ::std::thread;

#[repr(C)]
pub struct UartObj<'a>
{
    vrot_pe: &'a mut vrot_pe_lib::VrotPE,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pe_init(
    p_self: *mut UartObj,
    vrot: *mut vrot_pe_lib::VrotPE,
    _arg: *mut ffi::c_void,
) -> i32
{
    assert!(!p_self.is_null());

    let vrot_pe = unsafe{ vrot.as_mut().unwrap() };

    unsafe {
        ptr::write(p_self, UartObj { vrot_pe });
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pe_main(p_self: *mut UartObj)
{
    assert!(!p_self.is_null());
    let r_self = unsafe { p_self.as_mut() }.unwrap();
    let vrot_ptr_raw = r_self.vrot_pe as *mut vrot_pe_lib::VrotPE as usize;
    let vbus = unsafe {r_self.vrot_pe.pe_vrot_get_bus().unwrap().as_ref().unwrap()};

    let bus_interface_write = vbus.write;
    let bus_interface_read = vbus.read;
    let bus_interface_data_raw = vbus.data as usize;

    let msg_size = ::core::mem::size_of::<ffi::c_int>();

    //read from the bus
    let rx = thread::spawn(move ||{
        let vrot_ptr = vrot_ptr_raw as _;

        let rx_ptr = unsafe {
            vrot_pe_lib::VrotPE::pe_vrot_get_extra_var(
                vrot_ptr,
                vrot_pe_lib::VarType::Write,
                0,
            ) as *mut ffi::c_int
        };
        assert!(!rx_ptr.is_null());

        let ctx = bus_interface_data_raw as *const ();

        loop
        {
            let mut msg : ffi::c_int = 0;
            unsafe
            {
                (bus_interface_read)(ctx, &mut msg as *mut ffi::c_int as _, msg_size); //blocking
                println!("p2p uart read: {}", msg);
                *rx_ptr = msg;
                vrot_pe_lib::VrotPE::pe_vrot_trigger_hw_interrupt(
                    vrot_ptr,
                    0);
            }
        }
    });

    //write into the bus
    let tx = thread::spawn(move || {
        let tx_ptr = unsafe {
            vrot_pe_lib::VrotPE::pe_vrot_get_extra_var(
                vrot_ptr_raw as _,
                vrot_pe_lib::VarType::Read,
                0) as *mut ffi::c_int
        };
        assert!(!tx_ptr.is_null());

        let tx_send_trigger = unsafe {
            vrot_pe_lib::VrotPE::pe_vrot_get_extra_var(
                vrot_ptr_raw as _,
                vrot_pe_lib::VarType::Read,
                1) as *mut bool
        };


        if tx_send_trigger.is_null()
        {
            println!("p2p uart write: trigger NULL. end write thread");
            return;
        }

        let to_send = ::core::mem::size_of::<ffi::c_int>();

        loop
        {
            let ctx = bus_interface_data_raw as *const ();

            unsafe
            {
                println!("p2p uart write: waiting");
                if *tx_send_trigger
                {
                    println!("p2p uart write: writing: {}", *tx_ptr);
                    (bus_interface_write)(ctx, tx_ptr as _, to_send);
                    *tx_send_trigger = false;
                }
            }

            thread::sleep(std::time::Duration::from_millis(100));

        }
    });

    let _ = tx.join();
    let _ = rx.join();

    unreachable!();
}
