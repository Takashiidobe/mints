#![no_std]
extern crate alloc;
use alloc::vec::Vec;

use alloc::fmt;
use alloc::string::ToString as _;
use core::error::Error;
use core::ffi::{c_char, c_int, c_long};
use core::{ffi::CStr, fmt::Write, slice};

unsafe extern "C" {
    fn remove(path: *const c_char) -> c_int;
    fn write(fd: c_int, buf: *const u8, len: usize) -> isize;
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

pub fn parse_args(argc: c_int, argv: *const *const c_char) -> Vec<&'static str> {
    let args = unsafe { slice::from_raw_parts(argv, argc as usize) };
    args.into_iter()
        .map(|s| unsafe { CStr::from_ptr(s.clone()) }.to_str().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
pub enum RemoveFileError<'a> {
    FileNotFound(&'a str),
    AccessDenied(&'a str),
    Unknown(c_long, &'a str),
}

impl fmt::Display for RemoveFileError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RemoveFileError::FileNotFound(path) => writeln!(f, "File not found: {path}"),
            RemoveFileError::AccessDenied(path) => {
                writeln!(f, "Access denied: when trying to delete {path}")
            }
            RemoveFileError::Unknown(code, path) => {
                writeln!(f, "Unknown error code: when {code} trying to delete {path}",)
            }
        }
    }
}

impl Error for RemoveFileError<'_> {}

// Return codes are:
// 0   E_OK     No error
// -33 EFILNF 	File not found
// -36 EACCDN 	Access denied
pub fn remove_file<'a>(path: &'a str) -> Result<(), RemoveFileError<'a>> {
    let ret = unsafe { remove(path.as_ptr() as *const c_char) };
    match ret {
        0 => Ok(()),
        -33 => Err(RemoveFileError::FileNotFound(path)),
        -36 => Err(RemoveFileError::AccessDenied(path)),
        _ => Err(RemoveFileError::Unknown(ret, path)),
    }
}
