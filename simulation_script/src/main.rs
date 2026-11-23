use ::core::time;
use ::core::mem;
use ::core::ffi;

use ::std::path;
use ::std::fs;
use ::std::sync;
use ::std::sync::{atomic, atomic::Ordering};

#[sim::board_function]
unsafe extern "C" fn log_function_server(raw_who: u64, raw_str: u64)
{
    let ctx = unsafe { sim::get_ctx() };
    let mut msg = String::new();
    let mut guard = FD_SERVER.lock().unwrap();

    let raw_who_ptr = ctx.convert_ptr::<ffi::c_char>(raw_who);
    let raw_str_ptr = ctx.convert_ptr::<ffi::c_char>(raw_str);

    let cstr = unsafe { ffi::CStr::from_ptr(raw_str_ptr) };
    let cwho = unsafe { ffi::CStr::from_ptr(raw_who_ptr) };

    let args = format_args!("{}:{}\n", cwho.to_string_lossy(), cstr.to_string_lossy());

    {
        use ::std::fmt::Write;
        let _ = msg.write_fmt(args);
    }

    {
        use ::std::io::Write;
        let _ = guard.write_all(msg.as_bytes());
    }
}

#[sim::board_function]
unsafe extern "C" fn log_function_client(raw_who: u64, raw_str: u64)
{
    let ctx = unsafe { sim::get_ctx() };
    let mut msg = String::new();
    let mut guard = FD_CLIENT.lock().unwrap();

    let raw_who_ptr = ctx.convert_ptr::<ffi::c_char>(raw_who);
    let raw_str_ptr = ctx.convert_ptr::<ffi::c_char>(raw_str);

    let cstr = unsafe { ffi::CStr::from_ptr(raw_str_ptr) };
    let cwho = unsafe { ffi::CStr::from_ptr(raw_who_ptr) };

    let args = format_args!("{}:{}\n", cwho.to_string_lossy(), cstr.to_string_lossy());

    {
        use ::std::fmt::Write;
        let _ = msg.write_fmt(args);
    }

    {
        use ::std::io::Write;
        let _ = guard.write_all(msg.as_bytes());
    }
}

#[sim::board_function]
unsafe extern "C" fn server_uart_write(msg: ffi::c_int) -> ffi::c_int
{
    let ctx = unsafe { sim::get_ctx() };
    eprintln!("!!! RUST SERVER UART WRITE CALLED");

    let uart_buffer_ptr_ptr : *mut atomic::AtomicI32 = ctx.get_var_ptr("server/src/main.c", "uart_buffer");
    let uart_trigger : *mut atomic::AtomicBool = ctx.get_var_ptr("server/src/main.c", "trigger");

    unsafe
    {
        (*uart_buffer_ptr_ptr).store(msg, Ordering::Release);
        (*uart_trigger).store(true, Ordering::Release);
    }

    mem::size_of_val(&msg) as _
}

fn main() -> sim::SimulatorResult<()>
{
    sim::set_log_level(sim::Level::Trace);
    let mut simulator = sim::Simulator::init()?;

    simulator.add_package(path::Path::new("pkgs/rtc_timer.tar"))?;
    simulator.add_package(path::Path::new("pkgs/p2p_bus.tar"))?;
    simulator.add_package(path::Path::new("pkgs/p2p_uart.tar"))?;
    simulator.add_package(path::Path::new("pkgs/server.tar"))?;
    simulator.add_package(path::Path::new("pkgs/client.tar"))?;
    println!("pkgs loaded");

    simulator.create_bus("P2P_Bus", "uart_bus")?;
    simulator.attach_bus_to_board("server", "Uart", "uart_bus")?;
    simulator.attach_bus_to_board("client", "Uart", "uart_bus")?;

    {
        //server read from bus
        simulator.set_peripheral_extra_var_to_board_var(
            "server",
            "server/src/main.c",
            "uart_buffer",
            "Uart",
            vrot_pe_lib::VarType::Write,
            0,
        )?;


        //server write to bus
        simulator.set_peripheral_extra_var_to_board_var(
            "server",
            "server/src/main.c",
            "uart_buffer",
            "Uart",
            vrot_pe_lib::VarType::Read,
            0,
        )?;

        //server trigger
        simulator.set_peripheral_extra_var_to_board_var(
            "server",
            "server/src/main.c",
            "trigger",
            "Uart",
            vrot_pe_lib::VarType::Read,
            1,
        )?;
    }

    {
        //client read from bus
        simulator.set_peripheral_extra_var_to_board_var(
            "client",
            "client/src/main.c",
            "uart_buffer",
            "Uart",
            vrot_pe_lib::VarType::Write,
            0,
        )?;

        //client write to bus
        simulator.set_peripheral_extra_var_to_board_var(
            "client",
            "client/src/main.c",
            "uart_buffer",
            "Uart",
            vrot_pe_lib::VarType::Read,
            0,
        )?;
    }

    simulator.start_and_stop()?;

    //server
    {
        simulator.substitute_fun_with(
            "server",
            "log_function",
            sim::live_patching::Function::Function(log_function_server))?;

        simulator.substitute_fun_with(
            "server",
            "uart_write",
            sim::live_patching::Function::Function(server_uart_write))?;

    }

    // //client
    {
        simulator.substitute_fun_with(
            "client",
            "log_function",
            sim::live_patching::Function::Function(log_function_client))?;
    }


    println!("simulation running");
    simulator.continue_for(time::Duration::from_secs(6))?;


    println!("simulation ended");
    Ok(())
}



static FD_CLIENT : sync::LazyLock<sync::Mutex<fs::File>> = sync::LazyLock::new(||{
    const LOG_FILE : &str = "cluster_client.log";
    let _ = ::std::fs::remove_file(LOG_FILE);
    println!("output power board log file: {}", LOG_FILE);

    let fd = ::std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .unwrap();

    sync::Mutex::new(fd)
});

static FD_SERVER : sync::LazyLock<sync::Mutex<fs::File>> = sync::LazyLock::new(||{
    const LOG_FILE : &str = "cluster_server.log";
    let _ = ::std::fs::remove_file(LOG_FILE);
    println!("output power board log file: {}", LOG_FILE);

    let fd = ::std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .unwrap();

    sync::Mutex::new(fd)
});
