#![no_std] //No standart library  since it is kernel...
#![no_main] //No main for the same reason
use core::panic::PanicInfo;// Panic is unmanagable error

#[unsafe(no_mangle)] //Our own version of "main"
pub extern "C" fn _start() -> ! 
{
    loop {}
}

#[panic_handler] //The name suggests what it is
fn panic(_info: &PanicInfo) -> ! 
{
    loop {}
}


