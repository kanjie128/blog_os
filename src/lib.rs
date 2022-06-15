#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// exit form qemu
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xF4);
        port.write(exit_code as u32);
    }
}

/// marks all functions as testable, so we can deal with all tests using the same trait behavior.
pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

/// test_runner is called by `test_main` automatically.
/// it will call each function that marked with `#[test_case]`
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    // by default, custom_test_framework generate a `main` function that calls `test_runner`,
    // but `#![no_main]` tells rust to ignore it, so we use `reexport_test_harness_main` feature to
    // generate another function name called `test_main`
    test_main();
    loop {}
}

/// panic handler for `cargo test`
#[allow(clippy::empty_loop)]
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("[Error: {}]\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// panic in test, we want to see panic messages in host machine console,
/// so print everything to serial device to shown in console.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
