use std::convert::TryFrom;
use std::ffi::{CStr, CString, NulError};
use std::ptr::null_mut;

extern "C" {

    fn delete_char_arr(ptr: *const libc::c_char);

    fn error_type_to_string(error: libc::c_uint) -> *const libc::c_char;

    fn create_input_stream(str: *const libc::c_char) -> *mut libc::c_void;

}

/// Struct representing an input stream of bytes or string
#[repr(C)]
pub struct IInputStream {
    pub(crate) handler: *mut libc::c_void,
}

impl Default for IInputStream {
    /// synonym to [`IInputStream::null`]
    fn default() -> Self {
        Self::null()
    }
}

impl IInputStream {
    /// Creates a null stream, stands for no input at all
    pub fn null() -> Self {
        IInputStream {
            handler: null_mut(),
        }
    }
}

impl TryFrom<String> for IInputStream {
    type Error = NulError;
    /// Construct an input stream from a single string
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let cstring = CString::new(value)?;
        let stream = IInputStream {
            handler: unsafe { create_input_stream(cstring.as_ptr()) },
        };
        Ok(stream)
    }
}

/// Struct representing a common error within preprocessor
#[repr(C)]
pub struct TErrorInfo {
    m_type: libc::c_uint,
    m_line: libc::size_t,
}

impl TErrorInfo {
    /// Get the line number of this error
    pub fn get_line(&self) -> usize {
        self.m_line
    }

    /// Get the lint message of this error, returns None if an encoding error occurs
    pub fn get_message(&self) -> Option<String> {
        println!("{}", self.m_type);
        Some(unsafe {
            let msg = error_type_to_string(self.m_type);
            let owned = CStr::from_ptr(msg).to_str().ok().map(|s| s.to_owned());
            delete_char_arr(msg);
            owned?
        })
    }
}
