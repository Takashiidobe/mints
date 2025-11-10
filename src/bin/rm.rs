#![no_std]
#![no_main]
extern crate alloc;

use core::panic::PanicInfo;

use core::ffi::{c_char, c_int};

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::any::Any;
use core::error::Error;
use core::hint::black_box;
use libc_alloc::LibcAlloc;
use mints::stdio::remove::Errno as _;
use mints::stdio::{RemoveFileError, remove_file};
use mints::{parse_args, printf, println};

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[unsafe(no_mangle)]
pub extern "C" fn main(
    argc: c_int,
    argv: *const *const c_char,
    _envp: *const *const c_char,
) -> i32 {
    let args = parse_args(argc, argv);

    for arg in &args[1..] {
        if let Err(e) = remove_file(arg) {
            println!("{:?}", 10);
            return e.errno();
        };
    }

    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
