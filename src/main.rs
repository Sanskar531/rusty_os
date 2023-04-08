#![no_std]
#![no_main]
// custom test framework because rust's native
// testing lib depneds on std lib
#![feature(custom_test_frameworks)]
// Specify that our test_runner function is the test_runner function
// declared in this scope
#![test_runner(rusty_os::test_runner)]
// Specify cargo test to generate a 'test_main' function instead of
// main function because we don't have a main function which hence
// forth makes use liable for specifiying the test_main function
// inside start which is our 'main' function.
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rusty_os::{println, print};
#[cfg(test)]
use rusty_os::exit_qemu;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("nice!");
    
    // exit if we are testing
    #[cfg(test)]
    exit_qemu(rusty_os::QemuExitCode::Success);

    loop {}
}
