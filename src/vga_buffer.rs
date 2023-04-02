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
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        // Shift bits to the left by 4 and then or it on the foreground
        // hence completing the second 8 bytes of data for the color codes.
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar{
    ascii_char: u8,
    color_code: ColorCode
}

// Is represented as chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT] 
// in memory instead of type Buffer
#[repr(transparent)]
struct Buffer{
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer{
    coloumn_position: usize,
    color_code: ColorCode,
    // is static because we know exactly where the vga address is to write
    // things to display on screen
    buffer: &'static mut Buffer,
}

impl Writer{
    fn write_byte(&self, byte: u8) -> (){
        match byte{
            b'\n' => self.new_line(),
            byte => {
                if self.coloumn_position >= BUFFER_WIDTH{
                    self.new_line();
                }


            }
        }
    }

    fn new_line(&self) -> (){
    
    }
}
