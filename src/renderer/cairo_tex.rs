use crate::renderer::box_render::draw_box_tex;
use cairo::{Context, Operator, Rectangle, Surface};
use cairo_sys::{cairo_image_surface_create, cairo_image_surface_get_data, enums::Format};
use cgmath::Matrix4;
use gl::GLTexture2D;

/// A cairo context that renders into an OpenGL texture.
#[derive(Debug)]
pub struct CairoTex {
    texture: GLTexture2D,
    surface: Surface,
    context: Context,
    width: f64,
    height: f64,
    resolution: f64,
    tex_width: i32,
    tex_height: i32,
}

impl CairoTex {
    /// Creates a new CairoText with a width, a height, and a scaling factor.
    pub fn new(width: f64, height: f64, resolution: f64) -> CairoTex {
        let texture = unsafe { GLTexture2D::new() };
        unsafe {
            texture.bind();
            texture.clamp_to_edge_linear();
        }

        let tex_width = (width * resolution) as i32;
        let tex_height = (height * resolution) as i32;

        let surface = unsafe {
            Surface::from_raw_none(cairo_image_surface_create(
                Format::ARgb32,
                tex_width,
                tex_height,
            ))
        };
        let context = Context::new(&surface);

        CairoTex {
            texture,
            context,
            surface,
            width,
            height,
            resolution,
            tex_width,
            tex_height,
        }
    }

    /// Returns the unscaled context size.
    pub fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    /// Returns the scaling factor.
    pub fn resolution(&self) -> f64 {
        self.resolution
    }

    /// Returns the cairo context.
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Clears the context and applies the scaling factor.
    pub fn clear(&self) {
        self.context.identity_matrix();
        self.context.scale(self.resolution, self.resolution);
        self.context.set_operator(Operator::Clear);
        self.context.rectangle(0., 0., self.width, self.height);
        self.context.paint_with_alpha(1.);
        self.context.set_operator(Operator::Over);
    }

    /// Copies the pixels into the OpenGL texture.
    pub fn commit(&self) {
        unsafe {
            let tex_data = cairo_image_surface_get_data(self.surface.to_raw_none());
            self.texture.bind();
            self.texture
                .load_image_raw(self.tex_width, self.tex_height, tex_data);
        }
    }

    /// Renders the OpenGL texture with the given projection matrix and location.
    pub fn render(&self, matrix: Matrix4<f32>, x: f64, y: f64, scale: f64) {
        unsafe {
            self.texture.activate(0);

            let rect = Rectangle {
                x,
                y,
                width: self.width * scale,
                height: self.height * scale,
            };
            draw_box_tex(matrix, rect);
        }
    }
}
