use cgmath::{Matrix4, Vector2};
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

    pub fn render(&self, _matrix: Matrix4<f32>, renderer: &mut Renderer) {
        let res = self.shell.run(|shell| {
            shell.for_each_surface(&mut |surface_h: SurfaceHandle, sx, sy| {
                let _ = surface_h.run(|surface| {
                    let (width, height) = surface.current_state().size();
                    let render_width = width * renderer.output.scale() as i32;
                    let render_height = height * renderer.output.scale() as i32;

                    let render_box =
                        Area::new(Origin::new(sx, sy), Size::new(render_width, render_height));

                    let transform = renderer.output.get_transform().invert();
                    let matrix = project_box(
                        render_box,
                        transform,
                        0.0,
                        renderer.output.transform_matrix(),
                    );
                    unsafe { GLVertexArray::unbind() };
                    if let Some(tex) = surface.texture() {
                        renderer.render_texture_with_matrix(&tex, matrix);
                        surface.send_frame_done(current_time());
                    } else {
                        warn!("Surface has no texture");
                    }
                });
            });
        });

        if let Err(err) = res {
            warn!("Error in render: {}", err);
        }
    }

    #[wlroots_dehandle(shell_surface, surface)]
    pub fn with_surface<T, F: FnOnce(&mut Surface) -> T>(&self, f: F) -> T {
        let shell = &self.shell;
        use shell as shell_surface;
        let shell_surface = shell_surface.surface();
        use shell_surface as surface;
        f(surface)
    }

    #[wlroots_dehandle(shell)]
    pub fn contains_point(&self, point: Vector2<f64>) -> bool {
        let shell_h = &self.shell;
        use shell_h as shell;
        shell.geometry().contains_point(point.x, point.y)
    }

    #[wlroots_dehandle(shell)]
    pub fn map_location(&self, point: Vector2<f64>) -> (f64, f64) {
        let shell_h = &self.shell;
        use shell_h as shell;
        let origin = shell.geometry().origin;
        (point.x - origin.x as f64, point.y - origin.y as f64)
    }
}
