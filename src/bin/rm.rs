#![no_std]
#![no_main]
extern crate alloc;

use core::panic::PanicInfo;

use alloc::ffi::CString;
use alloc::vec::Vec;
use core::error::Error;
use core::ffi::{CStr, c_char, c_int};
use core::slice;

use mints::{eprint, eprintln, parse_args, print, println, remove_file};

use alloc::boxed::Box;
use libc_alloc::LibcAlloc;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[unsafe(no_mangle)]
pub extern "C" fn main(
    argc: c_int,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> anyhow::Result<()> {
    let args = parse_args(argc, argv);
    for arg in &args[1..] {
        remove_file(arg);
    }
    Ok(())
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
