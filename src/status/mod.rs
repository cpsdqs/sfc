use crate::renderer::CairoTex;
use cairo::Context;
use cgmath::Matrix4;
use core::f64::consts::PI;
use std::fmt;

const STATUS_HEIGHT: f64 = 22.;

pub mod battery;

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
            indicators: vec![Box::new(battery::BatteryIndicator::new())],
        }
    }

    fn draw(&mut self) {
        let ctx = self.inner.context();
        let (width, height) = self.inner.size();

        self.inner.clear();

        ctx.set_source_rgba(0., 0., 0., 0.3);
        ctx.rectangle(0., 0., width, height);
        ctx.fill();

        let time = time::now();
        let time_text = format!("{}:{:02}", time.tm_hour, time.tm_min);

        ctx.set_source_rgba(1., 1., 1., 1.);
        ctx.arc(21., height / 2., 5., 0., PI * 2.);
        ctx.fill();

        ctx.set_font_size(12.);
        let font_extents = ctx.font_extents();
        let text_extents = ctx.text_extents(&time_text);
        ctx.move_to(
            width / 2. - text_extents.width / 2.,
            height / 2. - font_extents.descent + font_extents.height / 2.,
        );
        ctx.show_text(&time_text);

        let mut right_x = width - 16.;
        for indicator in self.indicators.iter_mut() {
            indicator.draw(ctx, &mut right_x, height);
        }

        self.inner.commit();
    }

    pub fn render(&mut self, matrix: Matrix4<f32>) {
        self.draw();
        self.inner.render(matrix, 0., 0., 1.);
    }
}
