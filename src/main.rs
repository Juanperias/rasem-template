#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() {}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    loop {}
}
