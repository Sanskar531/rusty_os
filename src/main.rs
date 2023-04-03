#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{Buffer, Color, ColorCode, Writer};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer {
        coloumn_position: 0,
        color_code: ColorCode::new(Color::Green, Color::DarkGray),
        // Buffer is represented in memory as a transparent data structure
        // Hence, in our case here in 0xb8000 it acts like an array
        // and arrays are represented by its first memory address which
        // in our case is 0xb8000.
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string("HELLO_WORLD");

    loop {}
}
