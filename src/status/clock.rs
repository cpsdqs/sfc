use crate::status::StatusIndicator;
use cairo::Context;

#[derive(Debug)]
pub struct ClockIndicator;

impl ClockIndicator {
    pub fn new() -> ClockIndicator {
        ClockIndicator
    }
}

impl StatusIndicator for ClockIndicator {
    fn draw(&mut self, ctx: &Context, right_inset: &mut f64, height: f64) {
        let time = time::now();
        let day = match time.tm_wday {
            0 => "Sun",
            1 => "Mon",
            2 => "Tue",
            3 => "Wed",
            4 => "Thu",
            5 => "Fri",
            6 => "Sat",
            _ => "Judgment Day",
        };
        let time_text = format!("{} {}:{:02}", day, time.tm_hour, time.tm_min);

        let font_extents = ctx.font_extents();
        let text_extents = ctx.text_extents(&time_text);
        *right_inset -= text_extents.width;
        ctx.move_to(
            *right_inset,
            height / 2. - font_extents.descent + font_extents.height / 2.,
        );
        ctx.show_text(&time_text);
    }
}
