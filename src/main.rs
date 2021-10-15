#![no_std]
#![no_main]
#![feature(
    custom_test_frameworks,
    abi_x86_interrupt,
    const_fn_fn_ptr_basics,
    const_panic,
    const_mut_refs,
    const_fn_trait_bound,
    async_closure,
)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use futures_util::Future;
use rust_os::{println, printsln};
use rust_os::memory;
use rust_os::task::executor::Executor;
use rust_os::task::Task;
use core::future;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::{VirtAddr, PhysAddr};
use x86_64::structures::paging::Translate;

entry_point!(kmain);

#[no_mangle]
fn kmain(boot_info: &'static BootInfo) -> ! {
    println!("Strike the Earth!");
    printsln!("Strike the Earth!");

    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    memory::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    // If we're in test mode, run the test main.
    #[cfg(test)]
    test_main();

    // Run the kernel task executor.
    let mut executor = Executor::new();
    //executor.spawn(Task::new(rust_os::task::keyboard::process_scancode_stream()));
    executor.spawn(Task::new(rust_os::task::shell::ksh_main()));
    executor.run();

    // loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    printsln!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
