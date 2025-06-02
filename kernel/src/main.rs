#![no_std] //No standart library  since it is kernel...
#![no_main] //No main for the same reason
use core::panic::PanicInfo;// Panic is unmanagable error

static HELLO: &[u16] = b"

 /$$   /$$           /$$ /$$                       
| $$  | $$          | $$| $$                       
| $$  | $$  /$$$$$$ | $$| $$  /$$$$$$              
| $$$$$$$$ /$$__  $$| $$| $$ /$$__  $$             
| $$__  $$| $$$$$$$$| $$| $$| $$  \ $$             
| $$  | $$| $$_____/| $$| $$| $$  | $$             
| $$  | $$|  $$$$$$$| $$| $$|  $$$$$$/             
|__/  |__/ \_______/|__/|__/ \______/              
                                                   
                                                   
                                                   
 /$$      /$$                     /$$       /$$ /$$
| $$  /$ | $$                    | $$      | $$| $$
| $$ /$$$| $$  /$$$$$$   /$$$$$$ | $$  /$$$$$$$| $$
| $$/$$ $$ $$ /$$__  $$ /$$__  $$| $$ /$$__  $$| $$
| $$$$_  $$$$| $$  \ $$| $$  \__/| $$| $$  | $$|__/
| $$$/ \  $$$| $$  | $$| $$      | $$| $$  | $$    
| $$/   \  $$|  $$$$$$/| $$      | $$|  $$$$$$$ /$$
|__/     \__/ \______/ |__/      |__/ \_______/|__/

";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! 
{
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() 
    {
        unsafe 
        {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler] //The name suggests what it is
fn panic(_info: &PanicInfo) -> ! 
{
    loop {}
}


