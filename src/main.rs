#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    vga_print("world");
    loop {}
}

pub fn vga_print(s: &str) {
    println!("hello {s}!\nOS dev in Rust\n");
    panic!("panic!!!");
}
