#![no_std]
#![no_main]
extern crate alloc;
use alloc::format;
use core::panic::PanicInfo;
use libc_alloc::LibcAlloc;

use alloc::ffi::CString;
use alloc::fmt::format;
use alloc::string::ToString;
use alloc::vec::{self, Vec};
use core::ffi::{CStr, c_char, c_int};
use core::fmt::{self, Write};
use core::slice;

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

#[unsafe(no_mangle)]
pub extern "C" fn main(
    argc: c_int,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> c_int {
    let args = parse_args(argc, argv);
    for arg in &args[1..] {
        let c_string = CString::new(*arg).unwrap();

        let ptr: *const c_char = c_string.as_ptr();
        unsafe { remove(ptr) };
    }
    0
}

fn parse_args(argc: c_int, argv: *const *const c_char) -> Vec<&'static str> {
    let args = unsafe { slice::from_raw_parts(argv, argc as usize) };
    args.into_iter()
        .map(|s| unsafe { CStr::from_ptr(s.clone()) }.to_str().unwrap())
        .collect()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
