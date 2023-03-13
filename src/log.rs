// machine generated, do not edit

#![allow(dead_code)]

/// Helper function to convert a C string to a rust string slice
#[inline]
fn c_char_ptr_to_rust_str(c_char_ptr: *const core::ffi::c_char) -> &'static str {
    let c_str = unsafe { core::ffi::CStr::from_ptr(c_char_ptr) };
    c_str
        .to_str()
        .expect("c_char_ptr contained invalid Utf8 Data")
}

pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn slog_func(
            tag: *const core::ffi::c_char,
            log_level: u32,
            log_item: u32,
            message: *const core::ffi::c_char,
            line_nr: u32,
            filename: *const core::ffi::c_char,
            user_data: *mut core::ffi::c_void,
        );
    }
}
pub fn func(
    tag: &str,
    log_level: u32,
    log_item: u32,
    message: &str,
    line_nr: u32,
    filename: &str,
    user_data: *mut core::ffi::c_void,
) {
    unsafe {
        let tmp_0 = std::ffi::CString::new(tag).unwrap();
        let tmp_3 = std::ffi::CString::new(message).unwrap();
        let tmp_5 = std::ffi::CString::new(filename).unwrap();
        ffi::slog_func(
            tmp_0.as_ptr(),
            log_level,
            log_item,
            tmp_3.as_ptr(),
            line_nr,
            tmp_5.as_ptr(),
            user_data,
        )
    }
}
