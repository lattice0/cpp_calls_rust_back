
fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/openvpnclient.cpp")
        .compile("libopenvpnclient.a");
}