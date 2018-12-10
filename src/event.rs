use cgmath::Vector2;
use wlroots::key_events as key;
use wlroots::pointer_events as pointer;
use wlroots::tablet_tool_events as tool;
use wlroots::touch_events as touch;

pub enum Event<'a> {
    PointerButton(&'a pointer::ButtonEvent),
    PointerMotion(&'a pointer::MotionEvent),
    PointerAbsMotion(&'a pointer::AbsoluteMotionEvent),
    PointerAxis(&'a pointer::AxisEvent),
    Key(&'a key::KeyEvent),
    TouchDown(&'a touch::DownEvent, Vector2<f64>),
    TouchUp(&'a touch::UpEvent),
    TouchMotion(&'a touch::MotionEvent, Vector2<f64>),
    TouchCancel(&'a touch::CancelEvent),
    TabletAxis(&'a tool::AxisEvent),
    TabletProximity(&'a tool::ProximityEvent),
    TabletTip(&'a tool::TipEvent),
    TabletButton(&'a tool::ButtonEvent),
}
