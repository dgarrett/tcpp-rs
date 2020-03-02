use libc::c_char;

extern "C" {
    fn process(data: *mut c_char) -> *mut c_char;
}

mod tcpp {
    use std::ffi::{CStr, CString};

    pub fn process(data: String) -> Option<String> {
        let cstring = CString::new(data).ok()?.into_raw();
        let result_raw = unsafe {
            CStr::from_ptr(crate::process(cstring))
        };
        let _ = unsafe {
            CString::from_raw(cstring) // freed after ffi cross-border (to prevent memory leak)
        };
        Some(result_raw.to_str().ok()?.to_owned())
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let str = String::from("\
        #define TCPP_VALUE 10\n\
        \
        int main(int * argc,char ** argv) {\n\
        #if (TCPP_VALUE < 5)\n\
            printf(\"Hello tcpp TCPP_VALUE\");\n
        #else\n\
            printf(\"Hello Greater tcpp TCPP_VALUE\");\n\
        #endif\n\
        }\
        ");
        eprintln!("{:?}", crate::tcpp::process(str));
    }

}
