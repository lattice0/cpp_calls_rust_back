use libc::{c_char, c_int, size_t, c_void};

extern "C" {
    pub fn cpp_new_some_class() -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn call_on_some_class(i: u32, some_class_instance: *mut c_void) {
        
}

pub struct SomeClass {

}

impl SomeClass {
    pub fn new_some_class()-> *mut c_void {
        unsafe {cpp_new_some_class()}
    }
}

fn main() {

}