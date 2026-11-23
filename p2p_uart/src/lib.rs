use ::core::ptr;

use ::std::ffi;
use ::std::thread;
use ::std::sync;

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
        vrot_pe_lib::VrotPE::pe_vrot_set_trap(vrot, _trigger_write, 0);
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

    let max_msg_size = 1024;

    //read
    let rx = thread::spawn(move ||{
        let vrot_ptr = vrot_ptr_raw as _;

        let rx_ptr = unsafe {
            vrot_pe_lib::VrotPE::pe_vrot_get_extra_var(
                vrot_ptr,
                vrot_pe_lib::VarType::Write,
                0,
            )
        };
        assert!(!rx_ptr.is_null());

        let rx_read_ptr = unsafe {
            vrot_pe_lib::VrotPE::pe_vrot_get_extra_var(
                vrot_ptr,
                vrot_pe_lib::VarType::Write,
                1,
            ) as *mut u32
        };
        assert!(!rx_read_ptr.is_null());

        let ctx = bus_interface_data_raw as *const ();

        unsafe
        {
            let read = (bus_interface_read)(ctx, rx_ptr, max_msg_size); //blocking
            *rx_read_ptr = read as u32;
            vrot_pe_lib::VrotPE::pe_vrot_trigger_hw_interrupt(
                vrot_ptr,
                0);
        }
    });

    //write
    let tx = thread::spawn(move || {
        let tx_ptr = unsafe {
            vrot_pe_lib::VrotPE::pe_vrot_get_extra_var(
                vrot_ptr_raw as _,
                vrot_pe_lib::VarType::Read,
                0)
        };
        assert!(!tx_ptr.is_null());

        let tx_to_send = unsafe {
            vrot_pe_lib::VrotPE::pe_vrot_get_extra_var(
                vrot_ptr_raw as _,
                vrot_pe_lib::VarType::Read,
                1,
            ) as *mut u32
        };
        assert!(!tx_to_send.is_null());

        let mutex = &*TX_COND_MUTEX as *const _ as *mut libc::pthread_mutex_t;
        let cond = &*TX_COND_WAIT as *const _ as *mut libc::pthread_cond_t;

        loop
        {
            unsafe {
                libc::pthread_mutex_lock(mutex);
                libc::pthread_cond_wait(cond, mutex);
                libc::pthread_mutex_unlock(mutex);
            }

            let ctx = bus_interface_data_raw as *const ();
            let to_send = unsafe{ *tx_to_send } as usize;

            unsafe {(bus_interface_write)(ctx, tx_ptr, to_send)};
        }
    });

    let _ = tx.join();
    let _ = rx.join();

    unreachable!();
}

extern "C" fn _trigger_write()
{
    let mutex = &*TX_COND_MUTEX as *const _ as *mut libc::pthread_mutex_t;
    let cond = &*TX_COND_WAIT as *const _ as *mut libc::pthread_cond_t;

    unsafe {
        libc::pthread_mutex_lock(mutex);
        libc::pthread_cond_signal(cond);
        libc::pthread_mutex_unlock(mutex);
    }
}


static TX_COND_MUTEX: sync::LazyLock<libc::pthread_mutex_t> = sync::LazyLock::new(|| {
    let mut mutex = unsafe { ::core::mem::zeroed() };
    unsafe { libc::pthread_mutex_init(&mut mutex, ptr::null_mut()) };

    mutex
});

static TX_COND_WAIT: sync::LazyLock<libc::pthread_cond_t> = sync::LazyLock::new(|| {
    let mut cond = unsafe { ::core::mem::zeroed() };
    unsafe { libc::pthread_cond_init(&mut cond, ptr::null_mut()) };

    cond
});
