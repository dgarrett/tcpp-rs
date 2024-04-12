use std::ffi::{CStr, CString};

/// Foreign functions interface (FFI) definitions
pub mod ffi;

#[repr(C)]
struct CallbackSite {
    pub error: *mut libc::c_void,
    pub include: *mut libc::c_void,
}

mod tcpp {
    use std::os::raw::c_char;

    use crate::{ffi, CallbackSite};

    extern "C" {
        pub(crate) fn process_with_specs(
            data: *mut c_char,
            callbacks: *const libc::c_void,
            error: unsafe extern "C" fn(*const CallbackSite, *const ffi::TErrorInfo),
            include: unsafe extern "C" fn(
                *const CallbackSite,
                *const libc::c_char,
                bool,
            ) -> *const libc::c_void,
        ) -> *mut c_char;

        pub(crate) fn process(data: *mut c_char) -> *mut c_char;
    }
}

/// Process c/cpp strings to its expanded form using the `tcpp` preprocessor directly.
///
/// This function call cannot handle any errors or inclusions and simply halts when
/// a preprocessor error was detected, which makes it not recommended to be used.
///
/// You should probably check [`process_with`] instead
pub fn process(data: String) -> Option<String> {
    let cstring = CString::new(data).ok()?.into_raw();
    let result_raw = unsafe { CStr::from_ptr(tcpp::process(cstring)) };
    let _ = unsafe {
        CString::from_raw(cstring) // freed after ffi cross-border (to prevent memory leak)
    };
    Some(result_raw.to_str().ok()?.to_owned())
}

unsafe extern "C" fn callback_error<T: FnMut(ffi::TErrorInfo)>(
    callbacks: *const CallbackSite,
    error: *const ffi::TErrorInfo,
) {
    let callbacks = std::ptr::read(callbacks);
    let closure = callbacks.error as *mut T;
    (*closure)(std::ptr::read(error));
}

unsafe extern "C" fn callback_include<F: FnMut(String, bool) -> ffi::IInputStream>(
    callbacks: *const CallbackSite,
    file: *const libc::c_char,
    boolean: bool,
) -> *const libc::c_void {
    let callbacks = std::ptr::read(callbacks);
    let closure = callbacks.include as *mut F;
    let file = CStr::from_ptr(file).to_str().unwrap().to_owned();
    (*closure)(file, boolean).handler
}

/// This function calls the `tcpp` preprocessor with two callback functions.
///
/// While performs the same as function [`process`], this functions accepts two
/// closures to handle errors and inclusion (i.e. `#include`) respectively.
/// which gives more flexibility and supports further multi-file processing
///
/// # Example
///
/// ```
/// use tcpp::*;
/// use tcpp::ffi::*;
///
/// fn main() {
///     // read content from source file
///     let content = String::from_utf8(std::fs::read("main.c").unwrap()).unwrap();
///     let result = process_with(content,
///         |error| { // error processor
///             panic!("Preprocessor error: {} at line {}"
///                     , error.get_message().unwrap() // get description of the error
///                     , error.get_line()); // get line number of the error
///         },
///         |_, _|  { // inclusion processor
///             // we just ignore inclusions and returns a default (null) stream
///             IInputStream::default()
///         });
/// }
/// ```
///
pub fn process_with<T, F>(data: String, error: T, include: F) -> Option<String>
where
    T: FnMut(ffi::TErrorInfo),
    F: FnMut(String, bool) -> ffi::IInputStream,
{
    let cstring = CString::new(data).ok()?.into_raw();
    let callbacks = Box::new(CallbackSite {
        include: Box::into_raw(Box::new(include)) as *mut _,
        error: Box::into_raw(Box::new(error)) as *mut _,
    });
    let result_raw = unsafe {
        CStr::from_ptr(tcpp::process_with_specs(
            cstring,
            Box::into_raw(callbacks) as *const _,
            callback_error::<T>,
            callback_include::<F>,
        ))
    };
    let _ = unsafe {
        CString::from_raw(cstring) // freed after ffi cross-border (to prevent memory leak)
    };
    Some(result_raw.to_str().ok()?.to_owned())
}

#[cfg(test)]
mod tests {
    use crate::ffi::IInputStream;
    use crate::*;

    // #[test]
    // fn it_works() {
    //     let str = String::from(
    //         "\
    //     #define TCPP_VALUE 10\n\
    //     \
    //     int main(int * argc,char ** argv) {\n\
    //     #if (TCPP_VALUE < 5)\n\
    //         printf(\"Hello tcpp TCPP_VALUE\");\n
    //     #else\n\
    //         printf(\"Hello Greater tcpp TCPP_VALUE\");\n\
    //     #endif\n\
    //     }\
    //     ",
    //     );
    //     eprintln!("{:?}", process(str));
    // }

    #[test]
    fn reports_error() {
        let str = String::from(
            "\
        #defined TCPP_VALUE 10\n\
        \
        int main(int * argc,char ** argv) {\n\
        #if (TCPP_VALUE < 5)\n\
            printf(\"Hello tcpp TCPP_VALUE\");\n
        #else\n\
            printf(\"Hello Greater tcpp TCPP_VALUE\");\n\
        #endif\n\
        }\
        ",
        );
        eprintln!(
            "{:?}",
            process_with(
                str,
                |err| {
                    eprintln!("error! {:?}", err.get_message());
                },
                |_, _| { IInputStream::default() }
            )
        );
    }
}
