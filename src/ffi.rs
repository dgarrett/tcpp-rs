use std::convert::TryFrom;
use std::ffi::{CStr, CString, NulError};
use std::ptr::null_mut;

use libc::c_char;

extern "C" {

    fn error_type_to_string(error: libc::c_uint) -> *const libc::c_char;

    fn create_input_stream(str: *const libc::c_char) -> *mut libc::c_void;

}

#[repr(C)]
pub struct IInputStream {
    pub(crate) handler : *mut libc::c_void
}

impl IInputStream {
    pub fn null() -> Self {
        IInputStream {
            handler: null_mut()
        }
    }
}

impl TryFrom<String> for IInputStream {

    type Error = NulError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let cstring = CString::new(value)?;
        let stream = IInputStream {
            handler: unsafe {
                create_input_stream(cstring.as_ptr())
            }
        };
        Ok(stream)
    }

}

#[repr(C)]
pub struct TErrorInfo {
    m_type: libc::c_uint,
    m_line: libc::size_t
}

impl TErrorInfo {

    pub fn get_line(&self) -> usize {
        self.m_line
    }

    pub fn get_message(&self) -> Option<String> {
        println!("{}", self.m_type);
        Some(unsafe {
            let msg = error_type_to_string(self.m_type);
            CStr::from_ptr(msg).to_str().ok()?.to_owned()
        })
    }

}
