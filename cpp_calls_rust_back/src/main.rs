use libc::{c_void};

extern "C" {
    pub fn cpp_new_some_class() -> *mut c_void;
    pub fn cpp_some_class_set_callback(instance: *mut c_void, callback_in_rust: *mut c_void);
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

    pub fn set_callback(parent: *mut c_void) {
        unsafe {cpp_some_class_set_callback()}
    }
}

fn main() {
    let s = SomeClass::new_some_class();
}