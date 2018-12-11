use crate::event::Event;
use crate::server::Server;
use crate::status::StatusBar;
use cgmath::Matrix4;

mod box_render;
mod cairo_tex;
mod home_bar;

pub use self::box_render::*;
pub use self::cairo_tex::*;
pub use self::home_bar::*;

// TODO: screen rotation with iio-sensor-proxy

#[derive(Debug)]
pub struct Renderer {
    width: f64,
    height: f64,
    resolution: f64,
    status_bar: StatusBar,
    home_bar_captured_events: bool,
}

impl Renderer {
    pub fn new(width: f64, height: f64, resolution: f64) -> Renderer {
        Renderer {
            width,
            height,
            resolution,
            status_bar: StatusBar::new(width, resolution),
            home_bar_captured_events: false,
        }
    }

    pub fn dimensions(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    pub fn resolution(&self) -> f64 {
        self.resolution
    }

    pub fn handle_event(&mut self, mut event: Event, _server: &mut Server) {
        match event {
            Event::TouchDown(_, ref mut pos) | Event::TouchMotion(_, ref mut pos) => {
                pos.x *= self.width;
                pos.y *= self.height;
            }
            _ => (),
        }
    }

    pub fn render(
        &mut self,
        matrix: Matrix4<f32>,
        renderer: &mut wlroots::Renderer,
        server: &mut Server,
    ) {
        renderer.clear([0.3, 0.3, 0.3, 1.]);

        self.render_spaces(matrix, renderer, server);
        self.status_bar.render(matrix);
    }

    fn render_spaces(
        &self,
        matrix: Matrix4<f32>,
        renderer: &mut wlroots::Renderer,
        server: &mut Server,
    ) {
        let space_order: Vec<_> = server.space_order().iter().map(|x| *x).collect();
        for space_id in space_order.iter().rev() {
            server.space_mut(*space_id).unwrap().render(matrix, self, renderer);
        }
    }
}
