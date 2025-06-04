#![no_std] // No standard library since it is a kernel...
#![no_main] // No main for the same reason

use core::panic::PanicInfo; // Panic is an unmanageable error

// b"<ascii art>" means that it is a byte string, aka a sequence of bytes,
// which means that each character becomes a byte
// and &[u8] means that it is only a reference to the unsigned 8-bit integers
static HELLO: &[u8] = br#"


  /$$$$$$            /$$ /$$                     /$$                    
 /$$__  $$          | $$|__/                    |__/                    
| $$  \ $$  /$$$$$$$| $$ /$$  /$$$$$$   /$$$$$$$ /$$ /$$$$$$$   /$$$$$$ 
| $$  | $$ /$$_____/| $$| $$ /$$__  $$ /$$_____/| $$| $$__  $$ /$$__  $$
| $$  | $$| $$      | $$| $$| $$  \ $$|  $$$$$$ | $$| $$  \ $$| $$  \ $$
| $$/$$ $$| $$      | $$| $$| $$  | $$ \____  $$| $$| $$  | $$| $$  | $$
|  $$$$$$/|  $$$$$$$| $$| $$| $$$$$$$/ /$$$$$$$/| $$| $$  | $$|  $$$$$$$
 \____ $$$ \_______/|__/|__/| $$____/ |_______/ |__/|__/  |__/ \____  $$
      \__/                  | $$                               /$$  \ $$
                            | $$                              |  $$$$$$/
                            |__/                               \______/ 
                          /$$$$$$   /$$$$$$                             
                         /$$__  $$ /$$__  $$                            
                        | $$  \ $$| $$  \__/                            
                        | $$  | $$|  $$$$$$                             
                        | $$  | $$ \____  $$                            
                        | $$  | $$ /$$  \ $$                            
                        |  $$$$$$/|  $$$$$$/                            
                         \______/  \______/                             

"#;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8; //Pointer to the VGA buffer(font and its color)
const BUFFER_WIDTH: usize = 80; // Number of text colums in the VGA buffer
const BUFFER_HEIGHT: usize = 60; // Number of text row in the same buffer

// Function for priting to ascii art to the screen
fn print_ascii_art(art: &[u8], start_row: usize, start_col: usize, color: u8) 
{
    let mut row = start_row;
    let mut col = start_col;

    for &byte in art.iter() //Print each byte(character) of the ascii art manually
    {
        //There is no guarantee that everyhting would work as indended(pointers point to the
        //right place etc)
        unsafe 
        {
            match byte 
            {
                b'\n' => //\n - newline character moves to the new line
                {
                    row += 1;
                    col = 0;
                    if row >= BUFFER_HEIGHT 
                    {
                        // Optionally handle scrolling or stop printing here
                        break;
                    }
                }
                _ => {
                    if row >= BUFFER_HEIGHT || col >= BUFFER_WIDTH 
                    {
                        break; // Outside of screen boundaries
                    }
                    let offset = (row * BUFFER_WIDTH + col) * 2;
                    //Put the characters in the first byte
                    *VGA_BUFFER.offset(offset as isize) = byte;
                    //Put color into the second one
                    *VGA_BUFFER.offset(offset as isize + 1) = color;
                    col += 1;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print_ascii_art(HELLO, 0, 0, 0x07); // Light cyan on black
    
    loop{}
}

#[panic_handler] // The name suggests what it is: handles panics
fn panic(_info: &PanicInfo) -> ! 
{
    loop {} // Infinite loop on panic
}

