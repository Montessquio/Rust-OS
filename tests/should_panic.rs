#![no_std]
#![no_main]

use rust_os::{exit_qemu, prints, printsln, QemuExitCode};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    prints!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    prints!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    printsln!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
