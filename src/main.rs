#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

/// panic in our QEMU vm, so panic message should be shown in vm screen
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// panic when `cargo test`, so print everything to serial to console
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

pub fn vga_print(s: &str) {
    println!("hello {s}!\nOS dev in Rust\n");
    panic!("panic!!!");
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    // conditional compilation
    // test_main will be called only in `test` context
    #[cfg(test)]
    // by default, custom_test_framework generate a `main` function that calls `test_runner`,
    // but `#![no_main]` tells rust to ignore it, so we use `reexport_test_harness_main` feature to
    // generate another function name called `test_main`
    test_main();

    vga_print("world");

    loop {}
}

/// `#[test_case]` attributed function will be called by `test_runner` automatically
#[test_case]
fn trivial_assertion() {
    let one = 1;
    let two = 2;
    assert_eq!(one + one, two);
}
