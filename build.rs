use std::path::Path;
use std::env;

extern crate bt;
use bt::cmd;


fn main() {
	let mut cflags = env::var("CFLAGS").unwrap_or(String::new());

	let host = env::var("HOST").unwrap();

	let target = env::var("TARGET").unwrap();
	
	let msvc = host.contains("windows") && target.contains("windows");
	let mingw = !msvc && target.contains("windows");




	if target.contains("i686") {
		cflags.push_str(" -m32");
	} else if target.contains("x86_64") {
		cflags.push_str(" -m64");
	}
	if !target.contains("i686") {
		cflags.push_str(" -fPIC");
	}


	let dst = env::var("OUT_DIR").unwrap();
	let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();


	// Generate 
	cmd("cmake", |ref mut cmd| {
		let profile = match &env::var("PROFILE").unwrap()[..] {
			"bench" | "release" => "Release",
			_ => "Debug",
		};

		if msvc {
			// It's nessasary for 64bit build.
			// vs 12 2014 requires 'CMAKE_C_COMPILER'
			if target.contains("x86_64") { 
				cmd.arg("-G").arg("Visual Studio 14 2015 Win64");
			} else {
				cmd.arg("-G").arg("Visual Studio 14 2015");
			}
		} else if mingw {
			cmd.arg("-G").arg("MinGW Makefiles");
		}
		

		cmd
			.arg(&format!("-DCMAKE_BUILD_TYPE={}", profile))
			.arg(&format!("-DCMAKE_C_FLAGS={}", cflags))
			.arg("-DGLFW_BUILD_EXAMPLES=OFF")
			.arg("-DGLFW_BUILD_TESTS=OFF")
			.arg("-DGLFW_BUILD_DOCS=OFF")
			.arg(&format!("-DCMAKE_INSTALL_PREFIX={}", dst))
			.arg(src_dir)
			.current_dir(&Path::new(&dst));
	});



	// Build
	cmd("cmake", |ref mut cmd| {
		cmd
			.arg("--build").arg(".")
			.arg("--target").arg("install")
			.current_dir(&Path::new(&dst));
	});
	println!("cargo:root={}", dst);


	println!("cargo:rustc-link-search={}", &Path::new(&dst).join("lib").display());
    println!("cargo:rustc-link-lib=static=glfw3");
}