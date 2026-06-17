
// disables standard library
#![no_std]
// now we dont need crt0 to start runtime
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// we are now overwriting the operating system entry point with our own _start function:

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}