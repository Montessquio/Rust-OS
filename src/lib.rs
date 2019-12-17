//! Rust-OS

#![no_std]
//#![no_main]

/* These are my personal lints I use for every rust project. */
#![deny(bad_style,
        const_err,
        dead_code,
        improper_ctypes,
        missing_debug_implementations,
        missing_docs,
        no_mangle_generic_items,
        non_shorthand_field_patterns,
        overflowing_literals,
        path_statements ,
        patterns_in_fns_without_body,
        private_in_public,
        trivial_casts,
        trivial_numeric_casts,
        unconditional_recursion,
        unused,
        unused_extern_crates,
        unused_import_braces,
        unused_qualifications,
        unused_results,
        unused_allocation,
        unused_comparisons,
        unused_parens,
        while_true)]

use core::panic::PanicInfo;

extern crate x86;
#[macro_use]
extern crate lazy_static;
extern crate spin;

#[macro_use]
mod serial;

/// The OS long mode Rust entrypoint.
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    printsln!("Hello, World!");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}