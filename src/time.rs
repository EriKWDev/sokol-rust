// machine generated, do not edit

#![allow(dead_code)]


/// helper function to convert a C string to a rust string slice
#[inline]
fn c_char_ptr_to_rust_str(c_char_ptr: *const core::ffi::c_char) -> &'static str {
    let c_str = unsafe { core::ffi::CStr::from_ptr(c_char_ptr) };
    c_str.to_str().expect("c_char_ptr contained invalid Utf8 Data")
}

pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn stm_setup();
        pub fn stm_now() -> u64;
        pub fn stm_diff(new_ticks: u64, old_ticks: u64) -> u64;
        pub fn stm_since(start_ticks: u64) -> u64;
        pub fn stm_laptime(last_time: *mut u64) -> u64;
        pub fn stm_round_to_common_refresh_rate(frame_ticks: u64) -> u64;
        pub fn stm_sec(ticks: u64) -> f64;
        pub fn stm_ms(ticks: u64) -> f64;
        pub fn stm_us(ticks: u64) -> f64;
        pub fn stm_ns(ticks: u64) -> f64;
    }
}
pub fn setup() {
    unsafe {
        ffi::stm_setup()
    }
}
pub fn now() -> u64 {
    unsafe {
        ffi::stm_now()
    }
}
pub fn diff(new_ticks: u64, old_ticks: u64) -> u64 {
    unsafe {
        ffi::stm_diff(new_ticks, old_ticks)
    }
}
pub fn since(start_ticks: u64) -> u64 {
    unsafe {
        ffi::stm_since(start_ticks)
    }
}
pub fn laptime(last_time: &mut u64) -> u64 {
    unsafe {
        ffi::stm_laptime(last_time)
    }
}
pub fn round_to_common_refresh_rate(frame_ticks: u64) -> u64 {
    unsafe {
        ffi::stm_round_to_common_refresh_rate(frame_ticks)
    }
}
pub fn sec(ticks: u64) -> f64 {
    unsafe {
        ffi::stm_sec(ticks)
    }
}
pub fn ms(ticks: u64) -> f64 {
    unsafe {
        ffi::stm_ms(ticks)
    }
}
pub fn us(ticks: u64) -> f64 {
    unsafe {
        ffi::stm_us(ticks)
    }
}
pub fn ns(ticks: u64) -> f64 {
    unsafe {
        ffi::stm_ns(ticks)
    }
}
