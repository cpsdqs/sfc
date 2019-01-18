use crate::view::ViewID;
use cgmath::Vector2;
use std::collections::HashMap;
use wlroots::TouchId;

/// Event handling.
///
/// This exists because wlroots uses global-ish state for event handling.
pub struct SfSeat {
    kbd_focus: ViewID,
    ptr_focus: ViewID,
    touch_points: HashMap<TouchId, SfTouchPoint>,
}

struct SfTouchPoint {
    pos: Vector2<f64>,
    down: bool,
    focus: ViewID,
}

impl SfSeat {

}
