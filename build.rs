extern crate cmake;
use cmake::Config;

fn main() {
    let mut cfg = Config::new(".");

    cfg.define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_BUILD_DOCS", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib");

    let dst = if cfg!(feature = "wayland") {
        cfg.define("GLFW_BUILD_WAYLAND", "ON").build()
    } else {
        cfg.define("GLFW_BUILD_WAYLAND", "OFF").build()
    };

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=glfw3");
}
