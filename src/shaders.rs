use crate::{
    agnostic_interface::raylib_impl::RaylibShader,
    faux_quicksilver::{Transform, Vector},
};
use std::ffi::CStr;

extern "C" {
    pub fn glVertexAttrib2f(index: ::std::os::raw::c_uint, x: f32, y: f32);
}
extern "C" {
    pub fn glVertexAttrib3f(index: ::std::os::raw::c_uint, x: f32, y: f32, z: f32);
}
extern "C" {
    pub fn glVertexAttrib4f(index: ::std::os::raw::c_uint, x: f32, y: f32, z: f32, w: f32);
}
extern "C" {
    pub fn glGetAttribLocation(
        program: ::std::os::raw::c_uint,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

pub fn get_attrib_location(raylib_shader: &RaylibShader, name: &CStr) -> ::std::os::raw::c_uint {
    unsafe {
        glGetAttribLocation(raylib_shader.get_shader_id(), name.as_ptr()) as ::std::os::raw::c_uint
    }
}

pub fn set_transform_3f(index: ::std::os::raw::c_uint, transform: Transform) {
    // OpenGL stores matrix indices in column major order.
    for (i, idx) in (index..(index + 3)).enumerate() {
        unsafe {
            glVertexAttrib3f(
                idx,
                transform.mat[0 + i as usize],
                transform.mat[3 + i as usize],
                transform.mat[6 + i as usize],
            );
        }
    }
}

pub fn set_attr_2f(index: ::std::os::raw::c_uint, origin: Vector) {
    unsafe {
        glVertexAttrib2f(index, origin.x, origin.y);
    }
}
