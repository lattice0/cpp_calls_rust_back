#rustc --crate-type=staticlib interface.rs
g++ -o libsomeclass -L. someclass.cpp -pthread -ldl
#rustc --crate-type=cdylib interface.rs