
fn main() {
    cc::Build::new()
        .file("src/openvpnclient.cpp")
        .compile("libopenvpnclient.a");
}