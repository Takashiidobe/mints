#![no_std]

extern crate alloc;
pub mod stdio;

use alloc::vec::Vec;
use core::ffi::{c_char, c_int};
use core::fmt;
use core::{ffi::CStr, fmt::Write, slice};

unsafe extern "C" {
    pub fn write(fd: c_int, buf: *const u8, len: usize) -> isize;
    pub fn printf(fmt: *const c_char, ...);
    pub fn abort() -> !;
}

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
pub fn print_to(fd: i16, args: fmt::Arguments) {
    match args.as_str() {
        Some(s) => {
            let _ = Fd(fd).write_str(s);
        }
        None => panic!("non-const formatting is forbidden on this target"),
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::print_to(1, core::format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::print_to(1, core::format_args!("\r\n"));
    }};
    ($($arg:tt)*) => {{
        $crate::print_to(1, core::format_args!($($arg)*));
        $crate::print_to(1, core::format_args!("\r\n"));
    }}
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        $crate::print_to(2, core::format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! eprintln {
    () => {{
        $crate::print_to(2, "\r\n");
    }};
    ($($arg:tt)*) => {{
        $crate::print_to(2, core::format_args!($($arg)*));
        $crate::print_to(2, "\r\n");
    }}
}

pub fn parse_args(argc: c_int, argv: *const *const c_char) -> Vec<&'static str> {
    let args = unsafe { slice::from_raw_parts(argv, argc as usize) };
    args.into_iter()
        .map(|s| unsafe { CStr::from_ptr(s.clone()) }.to_str().unwrap())
        .collect()
}
