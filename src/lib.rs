// Two simple functions to test out FFI
// `no_mangle` attribute necessary to preserve symbols involved, as they need to be referenced from other languages
// `extern "C"` tells compiler to follow the C calling convention
#[no_mangle]
pub extern "C" fn double_input(x: i32) -> i32 {
    2 * x
}

// need to use CStr so both languages (i.e., Rust and Haskell) can interop
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn print_string(x: *const c_char) {
    unsafe {
        let cstring = CStr::from_ptr(x);
        if let Ok(input) = cstring.to_str() {
            println!("{}", input);
        } else {
            panic!("Unable to print input!");
        }
    }
}
