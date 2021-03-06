//! use serial device to write message from QEMU to host machine console.
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

/// print message to serial device, `SerialPort` already implement core::fmt::Write trait
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("printing to serial failed!");
}

#[macro_export]
macro_rules! serial_print {
	($($args:tt)*) => {
		$crate::serial::_print(format_args!($($args)*));
	};
}
#[macro_export]
macro_rules! serial_println {
	() => {
		$crate::serial_print!("\n");
	};
	($fmt:expr) => {
		$crate::serial_print!(concat!($fmt, "\n"));
	};
	($fmt:expr, $($args:tt)*) => {
		$crate::serial_print!(concat!($fmt, "\n"), $($args)*);
	};
}
