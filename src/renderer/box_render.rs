//! Utilities for rendering textured rectangles.

use cairo::Rectangle;
use cgmath::Matrix4;
use gl::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref BOX_BUFFER: Mutex<Option<GLBuffer<f32>>> = Mutex::new(None);
    static ref BOX_ARRAY: Mutex<Option<GLVertexArray>> = Mutex::new(None);
    static ref TEX_SHADER: Mutex<Option<GLProgram>> = Mutex::new(None);
}

const TEX_VERTEX: &str = "
#version 320 es
precision highp float;

uniform mat4 matrix;
in vec2 position;
uniform vec4 dimensions;
out vec2 tex_coord;

void main() {
    gl_Position = matrix * vec4(position * dimensions.zw + dimensions.xy, 0., 1.);
    tex_coord = position;
}
";

const TEX_FRAGMENT: &str = "
#version 320 es
precision highp float;

in vec2 tex_coord;
uniform sampler2D texture;
out vec4 out_color;

void main() {
    out_color = texture2D(texture, tex_coord).bgra;
}
";

/// Initializes the buffers and shaders for drawing rectangles.
///
/// Nothing will break when this is called twice but that’s probably not a very
/// useful thing to do.
pub unsafe fn init_box() {
    let box_buffer: GLBuffer<f32> = GLBuffer::new(GLBufferType::Array);
    box_buffer.bind();
    box_buffer.data(GLBufferUsage::StaticDraw, &[0., 0., 1., 0., 0., 1., 1., 1.]);

    let box_array = GLVertexArray::new();
    box_array.bind();
    box_array.enable_attrib(0);
    box_buffer.attrib_pointer(0, 2);
    box_buffer.unbind();
    GLVertexArray::unbind();

    *BOX_BUFFER.lock().unwrap() = Some(box_buffer);
    *BOX_ARRAY.lock().unwrap() = Some(box_array);

    let (tex_shader, ..) = create_vert_frag_program(TEX_VERTEX, TEX_FRAGMENT).unwrap();
    *TEX_SHADER.lock().unwrap() = Some(tex_shader);
}

/// Draws the rectangle, and assumes a shader was bound.
pub unsafe fn draw_box() {
    let box_array_ref = BOX_ARRAY.lock().unwrap();
    let box_array = box_array_ref.as_ref().unwrap();
    box_array.bind();
    gl::draw_arrays(GLDrawMode::TriangleStrip, 4);
}

/// Draws the rectangle with the texture shader, a projection matrix, and dimensions.
///
/// The texture should be bound to `TEXTURE0`.
pub unsafe fn draw_box_tex(matrix: Matrix4<f32>, rect: Rectangle) {
    let tex_shader_ref = TEX_SHADER.lock().unwrap();
    let tex_shader = tex_shader_ref.as_ref().unwrap();
    tex_shader.bind();
    tex_shader.uniform_vec4(
        "dimensions",
        rect.x as f32,
        rect.y as f32,
        rect.width as f32,
        rect.height as f32,
    );
    tex_shader.uniform_mat4("matrix", matrix.into());
    draw_box();
}
