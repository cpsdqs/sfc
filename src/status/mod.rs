use crate::renderer::CairoTex;
use cairo::Context;
use cgmath::Matrix4;
use std::f64::consts::PI;
use std::fmt;

const STATUS_HEIGHT: f64 = 22.;

pub mod battery;
pub mod clock;

#[derive(Debug)]
pub struct StatusBar {
    inner: CairoTex,
    indicators: Vec<Box<dyn StatusIndicator>>,
}

pub trait StatusIndicator: fmt::Debug {
    fn draw(&mut self, ctx: &Context, right_inset: &mut f64, height: f64);
}

impl StatusBar {
    pub fn new(width: f64, resolution: f64) -> StatusBar {
        StatusBar {
            inner: CairoTex::new(width, STATUS_HEIGHT, resolution),
            indicators: vec![
                Box::new(clock::ClockIndicator::new()),
                Box::new(battery::BatteryIndicator::new()),
            ],
        }
    }

    fn draw(&mut self) {
        let ctx = self.inner.context();
        let (width, height) = self.inner.size();

        self.inner.clear();

        ctx.set_source_rgba(0., 0., 0., 0.3);
        ctx.rectangle(0., 0., width, height);
        ctx.fill();

        ctx.set_source_rgba(1., 1., 1., 1.);
        ctx.set_font_size(12.);

        ctx.arc(21., height / 2., 5., 0., PI * 2.);
        ctx.fill();

        let mut right_x = width - 16.;
        for indicator in self.indicators.iter_mut() {
            indicator.draw(ctx, &mut right_x, height);
            right_x -= 8.;
        }

        self.inner.commit();
    }

    pub fn render(&mut self, matrix: Matrix4<f32>) {
        self.draw();
        self.inner.render(matrix, 0., 0., 1.);
    }
}
