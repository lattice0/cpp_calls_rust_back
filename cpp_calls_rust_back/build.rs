
fn main() {
    cc::Build::new()
        .file("src/someclass.cpp")
        .compile("libsomeclass.a");
}