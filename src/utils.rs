use cairo::Context;
use std::f64::consts::PI;

pub fn rounded_rect(ctx: &Context, x: f64, y: f64, width: f64, height: f64, radius: f64) {
    let right = x + width;
    let bottom = y + height;
    ctx.move_to(radius, y);
    ctx.line_to(right - radius, y);
    ctx.arc(right - radius, y + radius, radius, -PI / 2., 0.);
    ctx.line_to(right, bottom - radius);
    ctx.arc(right - radius, bottom - radius, radius, 0., PI / 2.);
    ctx.line_to(x + radius, bottom);
    ctx.arc(x + radius, bottom - radius, radius, PI / 2., PI);
    ctx.line_to(x, y + radius);
    ctx.arc(x + radius, y + radius, radius, PI, 3. * PI / 2.);
}
