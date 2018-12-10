use crate::event::Event;
use crate::server::Server;
use crate::spring::{RealTimeSpring, Spring};
use crate::status::StatusBar;
use cairo::LineCap;
use cgmath::Matrix4;
use gl::GLVertexArray;
use std::time::Instant;
use wlroots::utils::current_time;
use wlroots::{project_box, wlroots_dehandle, Area, Origin, Size, SurfaceHandle};

mod box_render;
mod cairo_tex;

pub use self::box_render::*;
pub use self::cairo_tex::*;

// TODO: screen rotation with iio-sensor-proxy

#[derive(Debug)]
pub struct Renderer {
    width: f64,
    height: f64,
    resolution: f64,
    status_bar: StatusBar,
    home_bar: HomeBar,
}

impl Renderer {
    pub fn new(width: f64, height: f64, resolution: f64) -> Renderer {
        Renderer {
            width,
            height,
            resolution,
            status_bar: StatusBar::new(width, resolution),
            home_bar: HomeBar::new(width, height, resolution),
        }
    }

    pub fn handle_event(&mut self, mut event: Event, _server: &mut Server) {
        match event {
            Event::TouchDown(_, ref mut pos) | Event::TouchMotion(_, ref mut pos) => {
                pos.x *= self.width;
                pos.y *= self.height;
            }
            _ => (),
        }

        // FIXME: this is terrible event handling
        // TODO: send events to windows, too
        match event {
            Event::TouchDown(_, pos) => {
                if self.home_bar.should_capture_event(pos.x, pos.y) {
                    self.home_bar.has_captured_events = true;
                    self.home_bar.handle_event(event);
                }
            }
            Event::TouchMotion(..) => {
                if self.home_bar.has_captured_events {
                    self.home_bar.handle_event(event);
                }
            }
            Event::TouchUp(..) => {
                if self.home_bar.has_captured_events {
                    self.home_bar.handle_event(event);
                    self.home_bar.has_captured_events = false;
                }
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

        self.render_views(renderer, server);
        self.status_bar.render(matrix);
        self.home_bar.render(matrix);
    }

    #[wlroots_dehandle(surface)]
    fn render_views(&self, renderer: &mut wlroots::Renderer, server: &mut Server) {
        for view in server.views.iter_mut().rev() {
            view.for_each_surface(&mut |surface_h: SurfaceHandle, sx, sy| {
                use surface_h as surface;

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
                renderer.render_texture_with_matrix(&surface.texture().unwrap(), matrix);
                surface.send_frame_done(current_time());
            });
        }
    }
}

#[derive(Debug)]
struct HomeBar {
    screen_height: f64,
    inner: CairoTex,
    has_captured_events: bool,
    touch_down_offset: f64,
    prev_touch_time: Instant,
    y_pos: RealTimeSpring,
}

const HOME_BAR_REGION_HEIGHT: f64 = 18.;

impl HomeBar {
    pub fn new(screen_width: f64, screen_height: f64, resolution: f64) -> HomeBar {
        HomeBar {
            screen_height,
            inner: CairoTex::new(screen_width, HOME_BAR_REGION_HEIGHT, resolution),
            has_captured_events: false,
            touch_down_offset: 0.,
            prev_touch_time: Instant::now(),
            y_pos: RealTimeSpring::new(Spring::new(1., 1.)),
        }
    }

    fn indicator_size(&self) -> (f64, f64) {
        let (width, _) = self.inner.size();

        let phi = 1.618;
        let indicator_width = width / (phi * phi * phi);

        (indicator_width, 3.)
    }

    pub fn should_capture_event(&self, x: f64, y: f64) -> bool {
        let (width, _) = self.inner.size();
        let (i_width, i_height) = self.indicator_size();
        if y < self.screen_height - HOME_BAR_REGION_HEIGHT / 2. - i_height * 2. {
            return false;
        }
        x > (width - i_width) / 2. && x < (width + i_width) / 2.
    }

    fn draw(&self) {
        let ctx = self.inner.context();
        let (width, height) = self.inner.size();

        self.inner.clear();

        let (indicator_width, _) = self.indicator_size();

        ctx.set_source_rgba(1., 1., 1., 0.5);
        ctx.set_line_cap(LineCap::Round);
        ctx.set_line_width(indicator_width.min(3.));
        ctx.new_path();
        ctx.move_to(width / 2. - indicator_width / 2., height / 2.);
        ctx.line_to(width / 2. + indicator_width / 2., height / 2.);
        ctx.stroke();

        self.inner.commit();
    }

    pub fn render(&mut self, matrix: Matrix4<f32>) {
        self.draw();
        let y_pos = if !self.has_captured_events {
            self.y_pos.update()
        } else {
            self.y_pos.update_time();
            self.y_pos.spring.value
        };
        self.inner.render(
            matrix,
            0.,
            self.screen_height - HOME_BAR_REGION_HEIGHT + y_pos,
            1.,
        );
    }

    fn map_touch_y(&self, y: f64) -> f64 {
        -(-(y - self.screen_height) / 2.).sqrt() * 2. - self.screen_height
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::TouchDown(_, pos) => {
                self.touch_down_offset = self.map_touch_y(pos.y) - self.y_pos.spring.value;
                self.y_pos.spring.velocity = 0.;
                self.prev_touch_time = Instant::now();
            }
            Event::TouchMotion(_, pos) => {
                let prev_value = self.y_pos.spring.value;
                self.y_pos.spring.value = self.map_touch_y(pos.y) - self.touch_down_offset;

                let delta = self.y_pos.spring.value - prev_value;
                let elapsed = self.prev_touch_time.elapsed();
                let elapsed_secs = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1e9;
                self.prev_touch_time = Instant::now();
                self.y_pos.spring.velocity = delta / elapsed_secs;
            }
            Event::TouchUp(_) => {}
            _ => (),
        }
    }
}
