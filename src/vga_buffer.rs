// Marking a variable as volatile stops compiler from optimizing it
// becuase all the compiler knows that we are writing somewhere but
// doesn't know that it has side-effects, here in our case the bytes
// stored in that buffer is renders to the screen. Hence, the compiler
// believes these writes are 'redundant' and hence optimizes the app
// and removes the writes as we are never reading from the buffer
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print(format_args!($($arg)*));
    };
}

pub fn _print(args: ::core::fmt::Arguments){
    use core::fmt::Write;

    let mut lock = WRITER.lock();
    lock.write_fmt(args).unwrap();
    drop(lock);
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        print!("{}\n", format_args!($($arg)*));
    };
}

lazy_static! {
    // using a spin lock here to wait until resource is open
    // in order to write to it again. Hence, we wrap it inside
    // a mutex here because we want it to be mutable. Hence, when
    // we wrap it in a spin lock, it will continuously be checking
    // where the resource is free or not using a tight loop. Hence,
    // when it is available.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        coloumn_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        // Buffer is represented in memory as a transparent data structure
        // Hence, in our case here in 0xb8000 it acts like an array
        // and arrays are represented by its first memory address which
        // in our case is 0xb8000.
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        // Shift bits to the left by 4 and then 'or' it on the foreground
        // hence completing the second 8 bits of data for the color codes to a byte.
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_char: u8,
    color_code: ColorCode,
}

// Is represented as chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
// in memory instead of type Buffer which is inline with the structure of
// the vga display buffer
#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub coloumn_position: usize,
    pub color_code: ColorCode,
    // is static because we know exactly where the vga address is to write
    // things to display on screen
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) -> () {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.coloumn_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.coloumn_position;

                // using volatile's api hence,
                // the compiler will not optimize
                // this write away
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                });
                self.coloumn_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for char in s.bytes() {
            match char {
                // Rust characters are utf-8 by default
                0x20..=0x7e | b'\n' => self.write_byte(char),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) -> () {
        // starts from 1 because we want to move the chars
        // from the next line up a line. Hence, we need to start at 1
        // in order to move line 1 to 0.
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let temp_value = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(temp_value);
            }
        }

        self.clear_line(BUFFER_HEIGHT - 1);
        self.coloumn_position = 0;
    }

    fn clear_line(&mut self, row: usize) {
        let black_color_code = ColorCode::new(Color::Black, Color::Black);

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_char: b' ',
                color_code: black_color_code,
            });
        }
    }
}
