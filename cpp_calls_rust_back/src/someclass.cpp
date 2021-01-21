#include <iostream>
#include <thread>

typedef int (*onDataCallback_)(uint8_t *data, size_t len);

class SomeClass {
    SomeClass() {

    }

    void run() {
        std::thread t([](){

        }, "Hello");
        t.join();
    }
}


extern "C" void* cpp_new_some_class() {
    return new SomeClass();
}