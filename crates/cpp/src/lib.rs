use std::os::raw::{c_char, c_int};

extern "C" {
    pub fn nodule_cpp_hello() -> c_int;
    pub fn nodule_cpp_version_name() -> *const c_char;
}

pub fn hello() -> i32 {
    unsafe { nodule_cpp_hello() }
}