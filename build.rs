extern crate cmake;
use cmake::Config;

fn main() {
    let mut cfg = Config::new("glfw");

    cfg.define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_BUILD_DOCS", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib");

    if cfg!(feature = "wayland") {
        cfg.define("GLFW_BUILD_WAYLAND", "ON");
        cfg.define("GLFW_BUILD_X11", "ON");
    } else {
        cfg.define("GLFW_BUILD_WAYLAND", "OFF");
        cfg.define("GLFW_BUILD_X11", "OFF");
    }

    println!(
        "cargo:rustc-link-search=native={}",
        cfg.build().join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=glfw3");
}
