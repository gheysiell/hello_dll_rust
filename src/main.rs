#[link(name = "hello_dll_rust.dll", kind="dylib")]
extern "C" {
    fn hello_world();
}

fn main() {
    unsafe {
        hello_world();
    }

    loop {}
}