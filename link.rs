#[link(name = "glfw3", kind = "static")]
extern {}

#[cfg(not(feature = "glfw-sys"))]
// leaving off `kind = static` allows for the specification of a dynamic library if desired
#[link(name = "glfw3")]
extern {}

#[cfg(target_os="windows")]
#[link(name = "opengl32")]
#[link(name = "gdi32")]
extern {}

#[cfg(target_os="linux")]
#[link(name = "X11")]
#[link(name = "GL")]
#[link(name = "Xxf86vm")]
#[link(name = "Xrandr")]
#[link(name = "Xi")]
#[link(name = "Xcursor")]
#[link(name = "Xinerama")]
extern {}

#[cfg(target_os="macos")]
#[link(name = "Cocoa", kind = "framework")]
#[link(name = "OpenGL", kind = "framework")]
#[link(name = "IOKit", kind = "framework")]
#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "QuartzCore", kind = "framework")]
extern {}