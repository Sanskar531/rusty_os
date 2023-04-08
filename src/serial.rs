use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    let mut lock = SERIAL1.lock();
    lock.write_fmt(args)
        .expect("Unable to write to serial port");
    drop(lock);
}

#[macro_export]
macro_rules!  serial_print{
    ($($args:tt)*) => {
        $crate::serial::_print(format_args!($($args)*));
    };
}

#[macro_export]
macro_rules!  serial_println{
    () => {
        serial_print!("\n");
    };
    ($fmt:expr) => {
        serial_print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($args:tt)*)=> {
        serial_print!(concat!($fmt, "\n"), $($args)*)
    };
}
