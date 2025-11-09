#![no_std]
#![no_main]
extern crate alloc;

use core::panic::PanicInfo;

use core::ffi::{c_char, c_int};

use libc_alloc::LibcAlloc;
use mints::parse_args;
use mints::stdio::remove_file;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[unsafe(no_mangle)]
pub extern "C" fn main(
    argc: c_int,
    argv: *const *const c_char,
    _envp: *const *const c_char,
) -> anyhow::Result<()> {
    let args = parse_args(argc, argv);
    for arg in &args[1..] {
        remove_file(arg)?;
    }
    Ok(())
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
