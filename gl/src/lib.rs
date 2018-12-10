//! OpenGL wrapper.

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[macro_use]
extern crate bitflags;

use gl::types::*;
use std::ffi::CString;
use std::os::raw;
use std::{marker, mem, ptr, slice};

/// Primitive OpenGL data types.
#[derive(Debug)]
pub enum GLType {
    /// An 8-bit signed byte.
    Byte,

    /// An 8-bit unsigned byte.
    UByte,

    /// A 16-bit signed short.
    Short,

    /// A 16-bit unsigned short.
    UShort,

    /// A 32-bit signed integer.
    Int,

    /// A 32-bit unsigned integer.
    UInt,

    /// A 16-bit half-float.
    HalfFloat,

    /// A 32-bit float.
    Float,

    /// A 32-bit float format that probably shouldn't be used.
    Fixed,
}

impl GLType {
    fn to_gl_const(&self) -> GLenum {
        match *self {
            GLType::Byte => gl::BYTE,
            GLType::UByte => gl::UNSIGNED_BYTE,
            GLType::Short => gl::SHORT,
            GLType::UShort => gl::UNSIGNED_SHORT,
            GLType::Int => gl::INT,
            GLType::UInt => gl::UNSIGNED_INT,
            GLType::HalfFloat => gl::HALF_FLOAT,
            GLType::Float => gl::FLOAT,
            GLType::Fixed => gl::FIXED,
        }
    }
}

/// An OpenGL buffer object.
///
/// The type specifies which type the buffer data has.
#[derive(Debug)]
pub struct GLBuffer<T> {
    buffer_type: GLBufferType,
    id: GLuint,
    phantom: marker::PhantomData<T>,
}

/// OpenGL buffer types.
#[derive(Debug)]
pub enum GLBufferType {
    /// A buffer used as a source for vertex data.
    Array,

    /// A type without semantics, for use with copying buffer sub data.
    CopyRead,

    /// A type without semantics, for use with copying buffer sub data.
    CopyWrite,

    /// A vertex index buffer.
    ElementArray,

    /// Used for async pixel transfer operations.
    PixelPack,

    /// Used for async pixel transfer operations.
    PixelUnpack,

    /// Used for transform feedback.
    TransformFeedback,

    /// Used for storing uniform blocks.
    Uniform,
}

impl GLBufferType {
    fn to_gl_const(&self) -> GLenum {
        match *self {
            GLBufferType::Array => gl::ARRAY_BUFFER,
            GLBufferType::CopyRead => gl::COPY_READ_BUFFER,
            GLBufferType::CopyWrite => gl::COPY_WRITE_BUFFER,
            GLBufferType::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
            GLBufferType::PixelPack => gl::PIXEL_PACK_BUFFER,
            GLBufferType::PixelUnpack => gl::PIXEL_UNPACK_BUFFER,
            GLBufferType::TransformFeedback => gl::TRANSFORM_FEEDBACK_BUFFER,
            GLBufferType::Uniform => gl::UNIFORM_BUFFER,
        }
    }
}

/// Usage types for buffer data.
#[derive(Debug)]
pub enum GLBufferUsage {
    /// The data will change frequently, and will be written to, but not read.
    StreamDraw,

    /// The data will change frequently, and will be read from, but not written.
    StreamRead,

    /// The data will change frequently, and won't be written to nor read from.
    StreamCopy,

    /// The data won't change, and will be written to, but not read.
    StaticDraw,

    /// The data won't change, and will be read from, but not written.
    StaticRead,

    /// The data won't change, and won't be written to nor read from.
    StaticCopy,

    /// The data will change sometimes, and will be written to, but not read.
    DynamicDraw,

    /// The data will change sometimes, and will be read from, but not written.
    DynamicRead,

    /// The data will change sometimes, and won't be written to nor read from.
    DynamicCopy,
}

impl GLBufferUsage {
    fn to_gl_const(&self) -> GLenum {
        match *self {
            GLBufferUsage::StreamDraw => gl::STREAM_DRAW,
            GLBufferUsage::StreamRead => gl::STREAM_READ,
            GLBufferUsage::StreamCopy => gl::STREAM_COPY,
            GLBufferUsage::StaticDraw => gl::STATIC_DRAW,
            GLBufferUsage::StaticRead => gl::STATIC_READ,
            GLBufferUsage::StaticCopy => gl::STATIC_COPY,
            GLBufferUsage::DynamicDraw => gl::DYNAMIC_DRAW,
            GLBufferUsage::DynamicRead => gl::DYNAMIC_READ,
            GLBufferUsage::DynamicCopy => gl::DYNAMIC_COPY,
        }
    }
}

impl<T> GLBuffer<T> {
    /// Creates a buffer with the specified type.
    pub unsafe fn new(buffer_type: GLBufferType) -> GLBuffer<T> {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);
        GLBuffer {
            buffer_type,
            id,
            phantom: marker::PhantomData,
        }
    }

    /// Binds the buffer.
    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.buffer_type.to_gl_const(), self.id);
    }

    /// Unbinds the buffer.
    pub unsafe fn unbind(&self) {
        gl::BindBuffer(self.buffer_type.to_gl_const(), 0);
    }

    /// Loads buffer data.
    /// For primitive types, use [data](#method.data) instead.
    pub unsafe fn data_raw(&self, usage: GLBufferUsage, data: *const raw::c_void, size: usize) {
        gl::BufferData(
            self.buffer_type.to_gl_const(),
            size as GLsizeiptr,
            data,
            usage.to_gl_const(),
        );
    }

    /// Binds this GLBuffer to the bound vertex array object at the specified
    /// index. For primitive types, use
    /// [attrib_pointer](#method.attrib_pointer) instead.
    pub unsafe fn attrib_pointer_raw(
        &self,
        index: GLuint,
        component_size: GLint,
        component_type: GLType,
    ) {
        gl::VertexAttribPointer(
            index,
            component_size,
            component_type.to_gl_const(),
            gl::FALSE,
            0,
            0 as *const raw::c_void,
        );
    }

    /// Deletes the buffer. To delete a buffer, you can also just drop it,
    /// which won't create a dangling pointer.
    pub unsafe fn delete(&mut self) {
        gl::DeleteBuffers(1, &self.id);
        self.id = 0;
    }
}

impl<T> GLBuffer<T>
where
    T: Sized,
{
    /// Loads the provided array as buffer data.
    pub unsafe fn data(&self, usage: GLBufferUsage, data: &[T]) {
        self.data_raw(
            usage,
            data as *const _ as *const raw::c_void,
            mem::size_of_val(data),
        );
    }
}

macro_rules! define_gl_buffer_type {
    ($type:ty, $gl_type:expr) => {
        impl GLBuffer<$type> {
            /// Binds the GLBuffer to the currently bound vertex array object at the
            /// specified index. Note that the specified attribute array must be
            /// enabled for this to have any effect.
            pub unsafe fn attrib_pointer(&self, index: GLuint, component_size: GLint) {
                self.attrib_pointer_raw(index, component_size, $gl_type);
            }
        }
    };
}

define_gl_buffer_type!(i8, GLType::Byte);
define_gl_buffer_type!(u8, GLType::UByte);
define_gl_buffer_type!(i16, GLType::Short);
define_gl_buffer_type!(u16, GLType::UShort);
define_gl_buffer_type!(i32, GLType::Int);
define_gl_buffer_type!(u32, GLType::UInt);
define_gl_buffer_type!(f32, GLType::Float);

impl<T> Drop for GLBuffer<T> {
    fn drop(&mut self) {
        unsafe { self.delete() };
    }
}

/// A vertex array object.
#[derive(Debug)]
pub struct GLVertexArray {
    id: GLuint,
}

impl GLVertexArray {
    /// Creates a VAO.
    pub unsafe fn new() -> GLVertexArray {
        let mut id: GLuint = 0;
        gl::GenVertexArrays(1, &mut id);
        GLVertexArray { id }
    }

    /// Binds the VAO.
    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }

    /// Unbinds any VAO.
    pub unsafe fn unbind() {
        gl::BindVertexArray(0);
    }

    /// Enables a vertex attribute array at the specified index.
    pub unsafe fn enable_attrib(&self, index: GLuint) {
        gl::EnableVertexAttribArray(index);
    }

    /// Disables a vertex attribute array at the specified index.
    pub unsafe fn disable_attrib(&self, index: GLuint) {
        gl::DisableVertexAttribArray(index);
    }

    /// Deletes the VAO. In most cases, to avoid dangling pointers, you should
    /// drop the VAO instead.
    pub unsafe fn delete(&mut self) {
        gl::DeleteVertexArrays(1, &self.id);
        self.id = 0;
    }
}

impl Drop for GLVertexArray {
    fn drop(&mut self) {
        unsafe { self.delete() };
    }
}

/// Shader types.
#[derive(Debug)]
pub enum GLShaderType {
    /// A vertex shader.
    Vertex,

    /// A fragment shader.
    Fragment,
}

impl GLShaderType {
    fn to_gl_const(&self) -> GLenum {
        match *self {
            GLShaderType::Vertex => gl::VERTEX_SHADER,
            GLShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

/// A single OpenGL shader.
#[derive(Debug)]
pub struct GLShader {
    id: GLuint,
}

impl GLShader {
    /// Loads a string as the shader source.
    ///
    /// # Panics
    /// Will panic if the shader source can't be converted into a CString.
    pub unsafe fn source(&self, shader_source: &str) {
        match CString::new(shader_source) {
            Ok(cstr) => gl::ShaderSource(self.id, 1, &cstr.as_ptr(), ptr::null::<GLint>()),
            Err(err) => panic!(err),
        }
    }

    /// Returns the shader compile status.
    pub unsafe fn is_compiled(&self) -> bool {
        let mut compiled: GLint = 0;
        gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut compiled);
        compiled == gl::TRUE as GLint
    }

    /// Returns the shader's info log.
    pub unsafe fn get_info_log(&self) -> String {
        let mut log_length: GLint = 0;
        gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut log_length);

        // allocate error string
        let mut bytes = Vec::with_capacity(log_length as usize);

        // fill bytes
        for _ in 0..log_length {
            bytes.push(0)
        }

        gl::GetShaderInfoLog(
            self.id,
            log_length,
            &mut log_length,
            bytes.as_mut_slice().as_mut_ptr(),
        );

        // String wants a u8, not an i8. [i8] can't be cast to [u8] so here's a hack
        let bytes_slice =
            slice::from_raw_parts(bytes.as_slice().as_ptr() as *const u8, bytes.len());
        String::from(String::from_utf8_lossy(bytes_slice))
    }

    /// Deletes the shader.
    pub unsafe fn delete(&mut self) {
        gl::DeleteShader(self.id);
        self.id = 0; // for now, just set it to zero
    }

    /// Compiles the shader.
    /// If it fails, the error string will contain the info log.
    pub unsafe fn compile(&self) -> Result<(), String> {
        gl::CompileShader(self.id);

        if !self.is_compiled() {
            Err(self.get_info_log())
        } else {
            Ok(())
        }
    }
}

impl Drop for GLShader {
    fn drop(&mut self) {
        unsafe { self.delete() };
    }
}

/// An OpenGL shader program.
#[derive(Debug)]
pub struct GLProgram {
    id: GLuint,
}

impl GLProgram {
    /// Attaches a shader.
    pub unsafe fn attach_shader(&self, shader: &GLShader) {
        gl::AttachShader(self.id, shader.id);
    }

    /// Binds the program.
    pub unsafe fn bind(&self) {
        gl::UseProgram(self.id);
    }

    /// Returns the program's link status.
    pub unsafe fn is_linked(&self) -> bool {
        let mut linked: GLint = 0;
        gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut linked);
        linked == gl::TRUE as GLint
    }

    /// Returns the program's info log.
    pub unsafe fn get_info_log(&self) -> String {
        let mut log_length: GLint = 0;
        gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut log_length);

        // allocate error string
        let mut bytes = Vec::with_capacity(log_length as usize);

        // fill bytes
        for _ in 0..log_length {
            bytes.push(0)
        }

        gl::GetProgramInfoLog(
            self.id,
            log_length,
            &mut log_length,
            bytes.as_mut_slice().as_mut_ptr(),
        );

        // String wants a u8, not an i8. [i8] can't be cast to [u8] so here's a hack
        let bytes_slice =
            slice::from_raw_parts(bytes.as_slice().as_ptr() as *const u8, bytes.len());
        String::from(String::from_utf8_lossy(bytes_slice))
    }

    /// Deletes the shader program.
    pub unsafe fn delete(&mut self) {
        gl::DeleteProgram(self.id);
        self.id = 0;
    }

    /// Links the shader program.
    /// If it fails, the error string will contain the info log.
    pub unsafe fn link(&self) -> Result<(), String> {
        gl::LinkProgram(self.id);

        if !self.is_linked() {
            Err(self.get_info_log())
        } else {
            Ok(())
        }
    }

    // TODO: cache uniform locations
    /// Returns a given uniform's location.
    pub unsafe fn get_uniform_location(&self, name: &str) -> GLint {
        let name = CString::new(name).unwrap();
        gl::GetUniformLocation(self.id, name.as_ptr())
    }

    /// Sets a uniform float.
    pub unsafe fn uniform_float(&self, name: &str, value: f32) {
        gl::Uniform1f(self.get_uniform_location(name), value);
    }

    /// Sets a uniform bool.
    pub unsafe fn uniform_bool(&self, name: &str, value: bool) {
        gl::Uniform1i(
            self.get_uniform_location(name),
            if value { gl::TRUE } else { gl::FALSE } as i32,
        );
    }

    /// Sets a uniform int.
    pub unsafe fn uniform_int(&self, name: &str, value: i32) {
        gl::Uniform1i(self.get_uniform_location(name), value);
    }

    /// Sets a uniform vec2.
    pub unsafe fn uniform_vec2(&self, name: &str, x: f32, y: f32) {
        gl::Uniform2f(self.get_uniform_location(name), x, y);
    }

    /// Sets a uniform vec3.
    pub unsafe fn uniform_vec3(&self, name: &str, x: f32, y: f32, z: f32) {
        gl::Uniform3f(self.get_uniform_location(name), x, y, z);
    }

    /// Sets a uniform vec4.
    pub unsafe fn uniform_vec4(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        gl::Uniform4f(self.get_uniform_location(name), x, y, z, w);
    }

    /// Sets a uniform mat4.
    pub unsafe fn uniform_mat4(&self, name: &str, arr: [[f32; 4]; 4]) {
        gl::UniformMatrix4fv(
            self.get_uniform_location(name),
            1,
            gl::FALSE,
            &arr as *const _ as *const f32,
        );
    }
}

impl Drop for GLProgram {
    fn drop(&mut self) {
        unsafe { self.delete() };
    }
}

#[derive(Debug)]
/// A two-dimensional texture.
pub struct GLTexture2D {
    id: GLuint,
}

impl GLTexture2D {
    /// Creates a texture.
    pub unsafe fn new() -> GLTexture2D {
        let mut id: GLuint = 0;
        gl::GenTextures(1, &mut id);
        GLTexture2D { id }
    }

    /// Binds the texture.
    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }

    /// Loads an RGBA image.
    pub unsafe fn load_image_raw(&self, width: i32, height: i32, buffer: *const u8) {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            width,
            height,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            buffer as *const GLvoid,
        );
    }

    /// Loads an RGBA image.
    pub unsafe fn load_image(&self, width: i32, height: i32, buffer: &[u8]) {
        self.load_image_raw(width, height, buffer as *const [u8] as *const u8);
    }

    /// Loads an RGB image.
    pub unsafe fn load_rgb_image(&self, width: i32, height: i32, buffer: &[u8]) {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            width,
            height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            buffer.as_ptr() as *const GLvoid,
        );
    }

    /// Loads a single-channel image.
    pub unsafe fn load_red_image(&self, width: i32, height: i32, buffer: &[u8]) {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as GLint,
            width,
            height,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            buffer.as_ptr() as *const GLvoid,
        );
    }

    /// Loads an empty image.
    pub unsafe fn load_null_image(&self, width: i32, height: i32) {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            width,
            height,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            0 as *const GLvoid,
        )
    }

    /// Activates the specified texture unit and binds this texture.
    pub unsafe fn activate(&self, index: u32) {
        gl::ActiveTexture(gl::TEXTURE0 + index);
        self.bind();
    }

    pub unsafe fn clamp_to_edge_linear(&self) {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    /// Deletes the texture.
    pub unsafe fn delete(&mut self) {
        gl::DeleteTextures(1, &self.id);
        self.id = 0;
    }
}

impl Drop for GLTexture2D {
    fn drop(&mut self) {
        unsafe { self.delete() };
    }
}

/// Framebuffer attachment types.
pub enum GLAttachmentType {
    /// A color buffer.
    Color(u8),

    /// A depth buffer.
    Depth,

    /// A stencil buffer.
    Stencil,
}

impl GLAttachmentType {
    fn is_color(&self) -> bool {
        match *self {
            GLAttachmentType::Color(_) => true,
            _ => false,
        }
    }
    fn to_gl_const(&self) -> GLenum {
        match *self {
            GLAttachmentType::Color(i) => gl::COLOR_ATTACHMENT0 + i as GLenum,
            GLAttachmentType::Depth => gl::DEPTH_ATTACHMENT,
            GLAttachmentType::Stencil => gl::STENCIL_ATTACHMENT,
        }
    }
}

#[derive(Debug)]
/// A framebuffer.
pub struct GLFramebuffer {
    id: GLuint,
}

impl GLFramebuffer {
    /// Binds the framebuffer, and makes the framebuffer be the target of all
    /// drawing operations.
    pub unsafe fn bind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
    }

    /// Unbinds the framebuffer.
    pub unsafe fn unbind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    /// Attaches a texture.
    pub unsafe fn texture_2d(&self, attachment: GLAttachmentType, texture: &GLTexture2D) {
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            attachment.to_gl_const(),
            gl::TEXTURE_2D,
            texture.id,
            0,
        );
    }

    /// Sets the draw buffer.
    pub unsafe fn draw_buffer(&self, draw_buffer: GLAttachmentType) {
        if !draw_buffer.is_color() {
            panic!("GL error: can't draw into non-color buffer");
        } else {
            gl::DrawBuffers(1, (&draw_buffer.to_gl_const()) as *const u32);
        }
    }

    /// Returns true if the frame buffer is complete.
    pub unsafe fn check_complete(&self) -> bool {
        gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE
    }

    /// Deletes the framebuffer.
    pub unsafe fn delete(&mut self) {
        gl::DeleteFramebuffers(1, &self.id);
        self.id = 0;
    }
}

impl Drop for GLFramebuffer {
    fn drop(&mut self) {
        unsafe { self.delete() };
    }
}

bitflags! {
    /// Clear masks.
    pub struct GLClearType: u8 {
        /// Clears the color buffer.
        const COLOR = 1;

        /// Clears the depth buffer.
        const DEPTH = 2;

        /// Clears the stencil buffer.
        const STENCIL = 4;
    }
}

/// Draw modes.
#[derive(Debug)]
pub enum GLDrawMode {
    /// Draws individual points where the vertices are.
    Points,

    /// Draws a line for each two vertices.
    Lines,

    /// Draws a continuous line, connecting all vertices from beginning to end.
    LineStrip,

    /// Draws a continuous line, connecting all vertices in a loop.
    LineLoop,

    /// Draws a triangle for each triplet of vertices.
    Triangles,

    /// Draws triangles in a zig-zag pattern with the vertices.
    TriangleStrip,

    /// Draws triangles with the first point and the subsequent points in a fan.
    TriangleFan,
}

impl GLDrawMode {
    fn to_gl_const(&self) -> GLenum {
        match *self {
            GLDrawMode::Points => gl::POINTS,
            GLDrawMode::Lines => gl::LINES,
            GLDrawMode::LineStrip => gl::LINE_STRIP,
            GLDrawMode::LineLoop => gl::LINE_LOOP,
            GLDrawMode::Triangles => gl::TRIANGLES,
            GLDrawMode::TriangleStrip => gl::TRIANGLE_STRIP,
            GLDrawMode::TriangleFan => gl::TRIANGLE_FAN,
        }
    }
}

#[derive(Debug)]
/// Test functions.
pub enum GLPassFunction {
    /// Never passes.
    Never,

    /// Will pass if it's closer.
    Less,

    /// Will pass if it's the same depth.
    Equal,

    /// Will pass if it's closer, or the same depth.
    Lequal,

    /// Will pass if it's further away.
    Greater,

    /// Will pass if it's not the same depth.
    Nequal,

    /// Will pass if it's further away, or the same depth.
    Gequal,

    /// Always passes.
    Always,
}

impl GLPassFunction {
    fn to_gl_const(&self) -> GLenum {
        match *self {
            GLPassFunction::Never => gl::NEVER,
            GLPassFunction::Less => gl::LESS,
            GLPassFunction::Equal => gl::EQUAL,
            GLPassFunction::Lequal => gl::LEQUAL,
            GLPassFunction::Greater => gl::GREATER,
            GLPassFunction::Nequal => gl::NOTEQUAL,
            GLPassFunction::Gequal => gl::GEQUAL,
            GLPassFunction::Always => gl::ALWAYS,
        }
    }
}

pub enum GLStencilOp {
    Replace,
    Keep,
    Zero,
    Incr,
    IncrWrap,
    Decr,
    DecrWrap,
    Invert,
}

impl GLStencilOp {
    fn to_gl_const(&self) -> GLenum {
        match *self {
            GLStencilOp::Replace => gl::REPLACE,
            GLStencilOp::Keep => gl::KEEP,
            GLStencilOp::Zero => gl::ZERO,
            GLStencilOp::Incr => gl::INCR,
            GLStencilOp::IncrWrap => gl::INCR_WRAP,
            GLStencilOp::Decr => gl::DECR,
            GLStencilOp::DecrWrap => gl::DECR_WRAP,
            GLStencilOp::Invert => gl::INVERT,
        }
    }
}

#[derive(Debug)]
/// Specifies the source blending factors.
pub enum GLBlendSource {
    /// Uses the source alpha values for blending.
    SrcAlpha,
}

impl GLBlendSource {
    fn to_gl_const(&self) -> GLenum {
        match *self {
            GLBlendSource::SrcAlpha => gl::SRC_ALPHA,
        }
    }
}

#[derive(Debug)]
/// Specifies the destination blending factors.
pub enum GLBlendDestination {
    /// Uses the inverse source alpha values for blending.
    OneMinusSrcAlpha,
}

impl GLBlendDestination {
    fn to_gl_const(&self) -> GLenum {
        match *self {
            GLBlendDestination::OneMinusSrcAlpha => gl::ONE_MINUS_SRC_ALPHA,
        }
    }
}

unsafe fn set_cap_enabled(cap: GLuint, enabled: bool) {
    if enabled {
        gl::Enable(cap);
    } else {
        gl::Disable(cap);
    }
}

/// Clears the specified buffers.
pub unsafe fn clear(clear_type: GLClearType) {
    let mut mask: GLbitfield = 0;
    if clear_type & GLClearType::COLOR == GLClearType::COLOR {
        mask |= gl::COLOR_BUFFER_BIT
    };
    if clear_type & GLClearType::DEPTH == GLClearType::DEPTH {
        mask |= gl::DEPTH_BUFFER_BIT
    };
    if clear_type & GLClearType::STENCIL == GLClearType::STENCIL {
        mask |= gl::STENCIL_BUFFER_BIT
    };
    gl::Clear(mask);
}

/// Sets the clear color.
pub unsafe fn clear_color(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) {
    gl::ClearColor(r, g, b, a);
}

/// Sets the viewport size.
pub unsafe fn viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    gl::Viewport(x, y, width, height);
}

/// Returns the viewport size in an array.
pub unsafe fn get_viewport() -> [GLint; 4] {
    let mut viewport: [GLint; 4] = [0, 0, 0, 0];
    gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr());
    viewport
}

/// Sets the viewport size from an array, for use with `get_viewport`.
pub unsafe fn set_viewport(viewport: [GLint; 4]) {
    gl::Viewport(viewport[0], viewport[1], viewport[2], viewport[3])
}

/// Enables or disables depth testing.
pub unsafe fn enable_depth_test(enabled: bool) {
    set_cap_enabled(gl::DEPTH_TEST, enabled);
}

/// Enables or disables face culling.
pub unsafe fn enable_cull_face(enabled: bool) {
    set_cap_enabled(gl::CULL_FACE, enabled);
}

/// Enables or disables blending.
pub unsafe fn enable_blend(enabled: bool) {
    set_cap_enabled(gl::BLEND, enabled);
}

/// Enables or disables dithering.
pub unsafe fn enable_dither(enabled: bool) {
    set_cap_enabled(gl::DITHER, enabled);
}

/// Enables or disables stencil testing.
pub unsafe fn enable_stencil(enabled: bool) {
    set_cap_enabled(gl::STENCIL_TEST, enabled);
}

/// Sets the depth test function.
pub unsafe fn set_depth_fn(function: GLPassFunction) {
    gl::DepthFunc(function.to_gl_const());
}

/// Sets the stencil test function.
pub unsafe fn set_stencil_fn(function: GLPassFunction, test_ref: GLuint, mask: GLuint) {
    gl::StencilFunc(function.to_gl_const(), test_ref as GLint, mask);
}

/// Sets the blending function.
pub unsafe fn set_blend_fn(source: GLBlendSource, dest: GLBlendDestination) {
    gl::BlendFunc(source.to_gl_const(), dest.to_gl_const());
}

/// Sets the stencil operation.
pub unsafe fn set_stencil_op(
    stencil_fail: GLStencilOp,
    depth_fail: GLStencilOp,
    pass: GLStencilOp,
) {
    gl::StencilOp(
        stencil_fail.to_gl_const(),
        depth_fail.to_gl_const(),
        pass.to_gl_const(),
    );
}

pub unsafe fn set_stencil_mask(mask: GLuint) {
    gl::StencilMask(mask);
}

/// Creates a shader.
pub unsafe fn create_shader(shader_type: GLShaderType) -> GLShader {
    GLShader {
        id: gl::CreateShader(shader_type.to_gl_const()),
    }
}

/// Creates a shader program.
pub unsafe fn create_program() -> GLProgram {
    GLProgram {
        id: gl::CreateProgram(),
    }
}

/// Creates a linked and compiled shader program from a vertex and fragment
/// source.
pub unsafe fn create_vert_frag_program(
    vertex_source: &str,
    fragment_source: &str,
) -> Result<(GLProgram, GLShader, GLShader), String> {
    let vertex = create_shader(GLShaderType::Vertex);
    vertex.source(vertex_source);
    let fragment = create_shader(GLShaderType::Fragment);
    fragment.source(fragment_source);

    let program = create_program();
    program.attach_shader(&vertex);
    program.attach_shader(&fragment);

    match vertex.compile().and(fragment.compile()).and(program.link()) {
        Ok(_) => Ok((program, vertex, fragment)),
        Err(err) => Err(err),
    }
}

/// Creates a frame buffer.
pub unsafe fn create_framebuffer() -> GLFramebuffer {
    let mut id: GLuint = 0;
    gl::GenFramebuffers(1, &mut id);
    GLFramebuffer { id }
}

/// Draws the bound VAO; see glDrawArrays.
pub unsafe fn draw_arrays(draw_mode: GLDrawMode, length: usize) {
    gl::DrawArrays(draw_mode.to_gl_const(), 0, length as GLsizei);
}

pub unsafe fn line_width(width: f32) {
    gl::LineWidth(width);
}
