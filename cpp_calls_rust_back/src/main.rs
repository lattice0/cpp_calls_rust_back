use libc::{c_void};
use ::std::*;

extern "C" {
    pub fn cpp_new_some_class() -> *mut c_void;
    pub fn cpp_some_class_set_callback(instance: *mut c_void, callback_in_rust: *mut c_void);
    pub fn cpp_some_class_set_rust_object(instance: *mut c_void, rust_object: *mut c_void);
}

unsafe extern "C" fn trampoline(this: *mut c_void, i: u32) {
    let some_class = &mut *(this as *mut SomeClass);

    //let string_argument = CStr::from_ptr(string_argument).to_string_lossy();
    some_class.do_something(i);
}

pub struct SomeClass(Box<Inner>);

struct Inner {
    cpp_some_class_pointer: *mut c_void
}

impl SomeClass {
    pub fn new_some_class()-> SomeClass {
        let mut s = SomeClass(
            Box::new(Inner{
                cpp_some_class_pointer: unsafe {cpp_new_some_class()}
            })
        );
        s.set_rust_object();
        s.set_callback(trampoline as *mut c_void);
        s
    }

    pub fn set_rust_object(&mut self) {
        unsafe{cpp_some_class_set_rust_object(self.0.cpp_some_class_pointer, &*self.0 as *const Inner as *mut c_void)}
    }

    pub fn set_callback(&mut self, parent: *mut c_void) {
        unsafe {cpp_some_class_set_callback(&*self.0 as *const Inner as *mut c_void, parent)}
    }

    pub fn do_something(&mut self, i: u32) {
        println!("{}", i);
    }
}

fn main() {
    let s = SomeClass::new_some_class();
    std::thread::sleep(std::time::Duration::from_secs(5));
}