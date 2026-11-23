use ::core::ptr;

use ::std::ffi;
use ::std::io;

#[repr(C)]
pub struct UartStar4Bus
{
    r_pipe: libc::c_int,
    w_pipe: libc::c_int,
}

impl UartStar4Bus
{
    /// # Safety
    /// name must be a valid c_string
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn init(p_self: *mut Self) -> i32
    {
        unsafe {
            let mut pipe : [ffi::c_int;2] = [0;2];

            if libc::pipe(pipe.as_mut_ptr()) == -1
            {
                return io::Error::last_os_error().raw_os_error().unwrap();
            }

            ptr::write(p_self, Self { r_pipe: pipe[0],  w_pipe: pipe[1] });

            0
        }
    }

    /// # Safety
    /// p_self must be a valid already initialized pointer
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn get_vtable(p_self: *const Self) -> vrot_bus_lib::VrotBusVtable
    {
        vrot_bus_lib::VrotBusVtable {
            read: uart_star_read,
            write: uart_star_write,
            data: p_self as *const (),
        }
    }

    /// # Safety
    /// p_self must be a valid already initialized pointer
    pub fn read(&self, o_buffer: *mut u8, o_buffer_size: usize) -> i32
    {
        println!("bus reading");
        let mut msg : libc::c_int = 0;
        if o_buffer.is_null() || o_buffer_size == 0
        {
            println!("bus read error");
            return -1;
        }

        unsafe
        {
            if libc::read(self.r_pipe, &mut msg as *mut ffi::c_int as _, o_buffer_size) == -1
            {
                println!("bus read error read");
                return -io::Error::last_os_error().raw_os_error().unwrap();
            }
            println!("bus read: {}, saving on: {:p}", msg, o_buffer);
            *(o_buffer as *mut ffi::c_int) = msg;
        }

        0
    }

    pub fn write(&self, o_buffer: *const u8, o_buffer_size: usize) -> i32
    {
        println!("bus writing");
        if o_buffer.is_null() || o_buffer_size == 0
        {
            println!("bus write error");
            return -1;
        }

        unsafe {
            if libc::write(self.w_pipe, o_buffer as _, o_buffer_size) == -1
            {
                println!("bus write error write");
                return -io::Error::last_os_error().raw_os_error().unwrap();
            }
        }

        0
    }
}

/// # Safety
/// p_self must be a valid already initialized pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uart_star_read(
    p_self: *const (),
    o_buffer: *mut u8,
    o_buffer_size: usize,
) -> i32
{
    let r_self = unsafe { (p_self as *const UartStar4Bus).as_ref() }.unwrap();
    r_self.read(o_buffer, o_buffer_size)
}

/// # Safety
/// p_self must be a valid already initialized pointer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uart_star_write(
    p_self: *const (),
    o_buffer: *const u8,
    o_buffer_size: usize,
) -> i32
{
    let r_self = unsafe { (p_self as *const UartStar4Bus).as_ref() }.unwrap();
    r_self.write(o_buffer, o_buffer_size)
}

