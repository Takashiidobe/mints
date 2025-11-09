#![no_std]
#![no_main]
extern crate alloc;
use core::panic::PanicInfo;
use libc_alloc::LibcAlloc;

use core::ffi::{CStr, c_char, c_int};
use core::fmt::{self, Write};

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

struct Fd(pub i16);

impl Write for Fd {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            write(self.0 as i32, s.as_ptr(), s.len() as usize);
        }
        Ok(())
    }
}

#[inline(always)]
fn _print_to(fd: i16, args: fmt::Arguments) {
    let _ = Fd(fd).write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::_print_to(1, core::format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\r\n") };
    ($($arg:tt)*) => {{
        $crate::_print_to(1, core::format_args!($($arg)*));
        $crate::_print_to(1, core::format_args!("\r\n"));
    }}
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        $crate::_print_to(2, core::format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! eprintln {
    () => { $crate::eprint!("\r\n") };
    ($($arg:tt)*) => {{
        $crate::_print_to(2, core::format_args!($($arg)*));
        $crate::_print_to(2, core::format_args!("\r\n"));
    }}
}

unsafe extern "C" {
    fn remove(path: *const c_char) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn write(fd: c_int, buf: *const u8, len: usize) -> isize;
}

use alloc::borrow::ToOwned as _;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_void;

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    unsafe {
        println!("{:?}, {:?}", argc, argv);
    }

    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
