use alloc::fmt;
use core::{
    error::Error,
    ffi::{c_char, c_int, c_long},
};

unsafe extern "C" {
    fn remove(path: *const c_char) -> c_int;
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
