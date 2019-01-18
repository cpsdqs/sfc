use crate::renderer::{init_box, Renderer as SfRenderer};
use crate::server::Server;
use cgmath::Matrix4;
use wlroots::*;

pub struct SfOutputHandler;

impl SfOutputHandler {
    pub fn new() -> SfOutputHandler {
        SfOutputHandler
    }
}

impl OutputHandler for SfOutputHandler {
    #[wlroots_dehandle(compositor, output)]
    fn on_frame(&mut self, compositor_handle: CompositorHandle, output_handle: OutputHandle) {
        use compositor_handle as compositor;
        use output_handle as output;

        // TODO: handle on_scale_change
        output.set_scale(2.);

        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        let renderer = compositor.renderer.as_mut().expect("no renderer");
        let mut wlr_renderer = renderer.render(output, None);

        let (width, height) = wlr_renderer.output.effective_resolution();
        let resolution = wlr_renderer.output.scale();

        if server.renderer.is_none() {
            let renderer = SfRenderer::new(width as f64, height as f64, resolution as f64);
            server.renderer = Some(renderer);
            unsafe { init_box() };
        }
        let matrix = wlr_renderer.output.transform_matrix();
        let matrix = Matrix4::from([
            [matrix[0] * resolution, matrix[3], matrix[6], 0.],
            [matrix[1], matrix[4] * resolution, matrix[7], 0.],
            [0., 0., 1., 0.],
            [matrix[2], matrix[5], matrix[8], 1.],
        ]);

        let mut renderer = server.renderer.take().unwrap();
        renderer.render(matrix, &mut wlr_renderer, server);
        server.renderer = Some(renderer);
    }
}
