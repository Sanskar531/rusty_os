#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::WRITER;
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("nice!");
    loop {}
}
