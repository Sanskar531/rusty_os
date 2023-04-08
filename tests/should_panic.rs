#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_os::{serial_println, serial_print, exit_qemu};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(rusty_os::QemuExitCode::Success);
    loop {}
}

fn should_panic(){
    panic!("WHAT!!");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Running should_panic::should_panic...\t");
    should_panic();

    loop {}
}
