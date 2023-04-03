const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

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
        // Shift bits to the left by 4 and then or it on the foreground
        // hence completing the second 8 bytes of data for the color codes.
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

// Is represented as chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
// in memory instead of type Buffer
#[repr(transparent)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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

                let row = BUFFER_HEIGHT - 2;
                let col = self.coloumn_position;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                };
                self.coloumn_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str){
        for char in s.bytes() {
            match char{
                0x20..=0x7e | b'\n' => self.write_byte(char),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&self) -> () {}
}
