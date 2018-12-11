use cgmath::Matrix4;
use gl::GLVertexArray;
use wlroots::utils::current_time;
use wlroots::*;

#[derive(Debug)]
pub struct View {
    pub shell: XdgV6ShellSurfaceHandle,
}

impl View {
    pub fn new(shell: XdgV6ShellSurfaceHandle) -> View {
        View { shell }
    }

    #[wlroots_dehandle(surface)]
    pub fn render(&self, matrix: Matrix4<f32>, renderer: &mut Renderer) {
        let shell = &self.shell;
        use shell as surface;

        surface.for_each_surface(&mut |surface_h: SurfaceHandle, sx, sy| {
            use surface_h as surface;

            let (width, height) = surface.current_state().size();
            let render_width = width * renderer.output.scale() as i32;
            let render_height = height * renderer.output.scale() as i32;

            let render_box = Area::new(Origin::new(sx, sy), Size::new(render_width, render_height));

            let transform = renderer.output.get_transform().invert();
            let matrix = project_box(
                render_box,
                transform,
                0.0,
                renderer.output.transform_matrix(),
            );
            unsafe { GLVertexArray::unbind() };
            renderer.render_texture_with_matrix(&surface.texture().unwrap(), matrix);
            surface.send_frame_done(current_time());
        });
    }
}
