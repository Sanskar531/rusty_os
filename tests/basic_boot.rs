#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rusty_os::{print, println, vga_buffer::{WRITER, BUFFER_HEIGHT}};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    // This removes this function when we
    // build our application becuase we
    // don't need tests for our product
    test_main();
    loop {}
}

#[test_case]
fn test_println(){
    print!("Hello!");
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 1..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_string_shows_on_screen() {
    let test_string = "This is the test string";
    print!("{}", test_string);

    let writer = WRITER.lock();
    for (i, c) in test_string.chars().enumerate() {
        let character = writer.buffer.chars[BUFFER_HEIGHT - 1][i].read();
        assert_eq!(c, char::from(character.ascii_char));
    }
    drop(writer);
}
