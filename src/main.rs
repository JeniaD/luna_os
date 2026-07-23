#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    luna_os::hlt_loop();
    // loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    luna_os::init();
    // x86_64::instructions::interrupts::int3();
    println!("Hello, \n  -- kernel");
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    luna_os::hlt_loop();
    //loop {}
}