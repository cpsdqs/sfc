use crate::event::Event;
use crate::renderer::cairo_tex::CairoTex;
use crate::spring::{RealTimeSpring, Spring};
use cairo::LineCap;
use cgmath::Matrix4;
use std::time::Instant;

#[derive(Debug)]
pub struct HomeBar {
    screen_height: f64,
    inner: CairoTex,
    touch_down_offset: f64,
    prev_touch_time: Instant,
    touch_down: bool,
    y_pos: RealTimeSpring,
}

const HOME_BAR_REGION_HEIGHT: f64 = 18.;

impl HomeBar {
    pub fn new(screen_width: f64, screen_height: f64, resolution: f64) -> HomeBar {
        HomeBar {
            screen_height,
            inner: CairoTex::new(screen_width, HOME_BAR_REGION_HEIGHT, resolution),
            touch_down_offset: 0.,
            prev_touch_time: Instant::now(),
            touch_down: false,
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
        let y_pos = if !self.touch_down {
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
                self.touch_down = true;
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
            Event::TouchUp(_) => {
                self.touch_down = false;
            }
            _ => (),
        }
    }
}
