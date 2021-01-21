use libc::{c_void};
use ::std::*;

extern "C" {
    pub fn cpp_new_some_class() -> *mut c_void;
    pub fn cpp_some_class_set_callback(instance: *mut c_void, callback_in_rust: *mut c_void);
    pub fn cpp_some_class_set_rust_object(instance: *mut c_void, rust_object: *mut c_void);
}

#[no_mangle]
pub extern "C" fn call_on_some_class(i: u32, some_class_instance: *mut c_void) {
     //Should cast some_class_instance to some_class_instance: Box<SomeClass> or some_class_instance: *mut SomeClass,
     //then we can call some_class_instance.do_something(i)
}

pub struct SomeClass {
    cpp_some_class_pointer: *mut c_void
}

impl SomeClass {
    pub fn new_some_class()-> SomeClass {
        let s = SomeClass{
            cpp_some_class_pointer: unsafe {cpp_new_some_class()}
        };
        s.set_rust_object();
        s.set_callback(?);
        s
    }

    pub fn set_rust_object() {
        unsafe{cpp_some_class_set_rust_object(self.cpp_some_class_pointer, ?)}
    }

    pub fn set_callback(parent: *mut c_void) {
        unsafe {cpp_some_class_set_callback(?,?)}
    }

    pub fn do_something(i: u32) {
        println!("{}", i);
    }
}

fn main() {
    let s = SomeClass::new_some_class();
    std::thread::sleep(std::time::Duration::from_secs(5));
}