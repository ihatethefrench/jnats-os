#![no_std] // don't link to rust standard library
#![no_main] // disable rust-level entry points ( like main() )
#![feature(custom_test_frameworks)] // enable using custom tests
#![test_runner(crate::test_runner)] // set test_runner() as the test function
#![reexport_test_harness_main = "test_main"] // make sure test function isnt called main

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;


#[no_mangle] // don't mangle the name of this function (keep it as _start)
pub extern "C" fn _start() -> ! {

    #[cfg(test)]
    test_main();

    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("err: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run (&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[test_case]
fn test_eq() {
    assert_eq!(1, 1);
}

#[test_case]
fn test_prln() {
    println!("test_prln out");
}

#[test_case]
fn test_mprln() {
    for _ in 0..200 {
        println!("test_mprln out");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
