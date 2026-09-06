//! GL function pointer loader and helpers.

#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code
)]

use std::sync::{Once, OnceLock};

pub struct GlFn<T: Copy>(OnceLock<T>);

impl<T: Copy> Default for GlFn<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy> GlFn<T> {
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }
    pub fn set(&self, val: T) {
        let _ = self.0.set(val);
    }
    pub fn get(&self) -> Option<T> {
        self.0.get().copied()
    }
    pub fn unwrap(&self) -> T {
        *self.0.get().expect("GL function not loaded")
    }
}

pub type GLenum = u32;
pub type GLuint = u32;
pub type GLint = i32;
pub type GLsizei = i32;
pub type GLboolean = u8;
pub type GLfloat = f32;
pub type GLchar = i8;
pub type GLbitfield = u32;
pub type GLsizeiptr = isize;

pub const GL_FALSE: GLboolean = 0;
pub const GL_TRUE: GLboolean = 1;
pub const GL_NO_ERROR: GLenum = 0;
pub const GL_TRIANGLES: GLenum = 0x0004;

pub const GL_TEXTURE_2D: GLenum = 0x0DE1;
pub const GL_TEXTURE0: GLenum = 0x84C0;
pub const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
pub const GL_TEXTURE_WRAP_T: GLenum = 0x2803;
pub const GL_LINEAR: GLint = 0x2601;
pub const GL_CLAMP_TO_EDGE: GLint = 0x812F;
pub const GL_NEAREST: GLint = 0x2600;

pub const GL_RGBA: GLenum = 0x1908;
pub const GL_RGBA8: GLenum = 0x8058;
pub const GL_RGBA16F: GLenum = 0x881A;
pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;

pub const GL_COLOR_BUFFER_BIT: GLbitfield = 0x4000;
pub const GL_FRAMEBUFFER: GLenum = 0x8D40;
pub const GL_READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub const GL_DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub const GL_COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const GL_FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;

pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
pub const GL_LINK_STATUS: GLenum = 0x8B82;
pub const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;

pub const GL_VIEWPORT: GLenum = 0x0BA2;
pub const GL_CURRENT_PROGRAM: GLenum = 0x8B8D;
pub const GL_ACTIVE_TEXTURE: GLenum = 0x84E0;
pub const GL_TEXTURE_BINDING_2D: GLenum = 0x8069;
pub const GL_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const GL_READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
pub const GL_DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
pub const GL_VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
pub const GL_ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub const GL_BLEND: GLenum = 0x0BE2;
pub const GL_DEPTH_TEST: GLenum = 0x0B71;
pub const GL_SCISSOR_TEST: GLenum = 0x0C11;
pub const GL_CULL_FACE: GLenum = 0x0B44;
pub const GL_STENCIL_TEST: GLenum = 0x0B90;

pub const GL_ARRAY_BUFFER: GLenum = 0x8892;
pub const GL_STATIC_DRAW: GLenum = 0x88E4;
pub const GL_FLOAT: GLenum = 0x1406;

macro_rules! gl_fn {
    ($name:ident, $ty:ty) => {
        pub static $name: GlFn<$ty> = GlFn::new();
    };
}

gl_fn!(glGenTextures, unsafe extern "C" fn(GLsizei, *mut GLuint));
gl_fn!(
    glDeleteTextures,
    unsafe extern "C" fn(GLsizei, *const GLuint)
);
gl_fn!(glBindTexture, unsafe extern "C" fn(GLenum, GLuint));
gl_fn!(
    glTexImage2D,
    unsafe extern "C" fn(
        GLenum,
        GLint,
        GLint,
        GLsizei,
        GLsizei,
        GLint,
        GLenum,
        GLenum,
        *const libc::c_void,
    )
);
gl_fn!(glTexParameteri, unsafe extern "C" fn(GLenum, GLenum, GLint));
gl_fn!(glActiveTexture, unsafe extern "C" fn(GLenum));
gl_fn!(
    glCopyTexSubImage2D,
    unsafe extern "C" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei)
);

gl_fn!(
    glGenFramebuffers,
    unsafe extern "C" fn(GLsizei, *mut GLuint)
);
gl_fn!(
    glDeleteFramebuffers,
    unsafe extern "C" fn(GLsizei, *const GLuint)
);
gl_fn!(glBindFramebuffer, unsafe extern "C" fn(GLenum, GLuint));
gl_fn!(
    glFramebufferTexture2D,
    unsafe extern "C" fn(GLenum, GLenum, GLenum, GLuint, GLint)
);
gl_fn!(
    glCheckFramebufferStatus,
    unsafe extern "C" fn(GLenum) -> GLenum
);
gl_fn!(
    glBlitFramebuffer,
    unsafe extern "C" fn(
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLint,
        GLbitfield,
        GLenum,
    )
);

gl_fn!(glCreateShader, unsafe extern "C" fn(GLenum) -> GLuint);
gl_fn!(glDeleteShader, unsafe extern "C" fn(GLuint));
gl_fn!(
    glShaderSource,
    unsafe extern "C" fn(GLuint, GLsizei, *const *const GLchar, *const GLint)
);
gl_fn!(glCompileShader, unsafe extern "C" fn(GLuint));
gl_fn!(
    glGetShaderiv,
    unsafe extern "C" fn(GLuint, GLenum, *mut GLint)
);
gl_fn!(
    glGetShaderInfoLog,
    unsafe extern "C" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar)
);
gl_fn!(glCreateProgram, unsafe extern "C" fn() -> GLuint);
gl_fn!(glDeleteProgram, unsafe extern "C" fn(GLuint));
gl_fn!(glAttachShader, unsafe extern "C" fn(GLuint, GLuint));
gl_fn!(glLinkProgram, unsafe extern "C" fn(GLuint));
gl_fn!(
    glGetProgramiv,
    unsafe extern "C" fn(GLuint, GLenum, *mut GLint)
);
gl_fn!(
    glGetProgramInfoLog,
    unsafe extern "C" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar)
);
gl_fn!(glUseProgram, unsafe extern "C" fn(GLuint));
gl_fn!(
    glGetUniformLocation,
    unsafe extern "C" fn(GLuint, *const GLchar) -> GLint
);
gl_fn!(glUniform1i, unsafe extern "C" fn(GLint, GLint));
gl_fn!(glUniform1f, unsafe extern "C" fn(GLint, GLfloat));
gl_fn!(glUniform2f, unsafe extern "C" fn(GLint, GLfloat, GLfloat));

gl_fn!(
    glGenVertexArrays,
    unsafe extern "C" fn(GLsizei, *mut GLuint)
);
gl_fn!(
    glDeleteVertexArrays,
    unsafe extern "C" fn(GLsizei, *const GLuint)
);
gl_fn!(glBindVertexArray, unsafe extern "C" fn(GLuint));

gl_fn!(glDrawArrays, unsafe extern "C" fn(GLenum, GLint, GLsizei));
gl_fn!(
    glViewport,
    unsafe extern "C" fn(GLint, GLint, GLsizei, GLsizei)
);

gl_fn!(glGetIntegerv, unsafe extern "C" fn(GLenum, *mut GLint));
gl_fn!(glGetError, unsafe extern "C" fn() -> GLenum);
gl_fn!(glEnable, unsafe extern "C" fn(GLenum));
gl_fn!(glDisable, unsafe extern "C" fn(GLenum));
gl_fn!(glIsEnabled, unsafe extern "C" fn(GLenum) -> GLboolean);
gl_fn!(glClear, unsafe extern "C" fn(GLbitfield));
gl_fn!(
    glClearColor,
    unsafe extern "C" fn(GLfloat, GLfloat, GLfloat, GLfloat)
);
gl_fn!(glFinish, unsafe extern "C" fn());

static INIT: Once = Once::new();

unsafe fn cast_gl_fn<T: Copy>(ptr: *mut libc::c_void) -> T {
    std::mem::transmute_copy::<*mut libc::c_void, T>(&ptr)
}

macro_rules! load {
    ($name:ident) => {
        let sym = concat!(stringify!($name), "\0");
        let p = crate::hook::real_dlsym(libc::RTLD_DEFAULT, sym.as_ptr() as *const _);
        if !p.is_null() {
            $name.set(cast_gl_fn(p));
        } else {
            let p2 = crate::hook::get_real_gl_proc(sym.as_ptr() as *const _);
            if !p2.is_null() {
                $name.set(cast_gl_fn(p2));
            }
        }
    };
}

pub fn ensure_loaded() {
    INIT.call_once(|| unsafe {
         // On Debian-based distros, GLFW loads GL libs via dlopen(RTLD_LOCAL),
        // hiding their symbols from dlsym(RTLD_DEFAULT). Promoting them to
        // RTLD_GLOBAL here ensures all subsequent lookups succeed.
        for lib in [
            b"libGL.so.1\0".as_ptr(),
            b"libGLX.so.0\0".as_ptr(),
            b"libGLdispatch.so.0\0".as_ptr(),
        ] {
            libc::dlopen(lib as *const libc::c_char, libc::RTLD_LAZY | libc::RTLD_GLOBAL);
        }
        load!(glGenTextures);
        load!(glDeleteTextures);
        load!(glBindTexture);
        load!(glTexImage2D);
        load!(glTexParameteri);
        load!(glActiveTexture);
        load!(glCopyTexSubImage2D);

        load!(glGenFramebuffers);
        load!(glDeleteFramebuffers);
        load!(glBindFramebuffer);
        load!(glFramebufferTexture2D);
        load!(glCheckFramebufferStatus);
        load!(glBlitFramebuffer);

        load!(glCreateShader);
        load!(glDeleteShader);
        load!(glShaderSource);
        load!(glCompileShader);
        load!(glGetShaderiv);
        load!(glGetShaderInfoLog);
        load!(glCreateProgram);
        load!(glDeleteProgram);
        load!(glAttachShader);
        load!(glLinkProgram);
        load!(glGetProgramiv);
        load!(glGetProgramInfoLog);
        load!(glUseProgram);
        load!(glGetUniformLocation);
        load!(glUniform1i);
        load!(glUniform1f);
        load!(glUniform2f);

        load!(glGenVertexArrays);
        load!(glDeleteVertexArrays);
        load!(glBindVertexArray);

        load!(glDrawArrays);
        load!(glViewport);

        load!(glGetIntegerv);
        load!(glGetError);
        load!(glEnable);
        load!(glDisable);
        load!(glIsEnabled);
        load!(glClear);
        load!(glClearColor);
        load!(glFinish);
    });
}

/// Look up a uniform location by name. Returns `-1` if not found.
pub fn get_uniform_location(program: u32, name: &str) -> i32 {
    let mut buf = name.as_bytes().to_vec();
    buf.push(0);
    unsafe {
        match glGetUniformLocation.get() {
            Some(f) => f(program, buf.as_ptr() as *const _),
            None => -1,
        }
    }
}

/// Set a float uniform.
pub fn uniform_1f(location: i32, value: f32) {
    unsafe {
        if let Some(f) = glUniform1f.get() {
            f(location, value);
        }
    }
}

/// Set an int uniform.
pub fn uniform_1i(location: i32, value: i32) {
    unsafe {
        if let Some(f) = glUniform1i.get() {
            f(location, value);
        }
    }
}
