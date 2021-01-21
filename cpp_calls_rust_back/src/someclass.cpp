#include <iostream>
#include <thread>
#include <chrono>

typedef void (*fptr)(int);

class SomeClass {
public: 
    SomeClass() {

    }

    void run() {
        std::thread t([this](){
            std::this_thread::sleep_for(std::chrono::milliseconds(2000));
            fptr rust_callback = reinterpret_cast<fptr>(reinterpret_cast<long>(this,this->callback_in_rust)) ;
            (rust_callback)(3);
        });
        t.join();
    }

    void* callback_in_rust;
};

extern "C" void* cpp_new_some_class() {
    return new SomeClass();
}

//Sets the Rust function `callback_in_rust` to be called by this SomeClass `instance`
extern "C" void* cpp_some_class_set_callback(void* instance, void* callback_in_rust) {
    ((SomeClass*)instance)->callback_in_rust = callback_in_rust;
}

//Runs the `run` function which delays a bit before calling back the rust function
extern "C" void* cpp_some_class_run(void* instance) {
    ((SomeClass*)instance)->run();
}