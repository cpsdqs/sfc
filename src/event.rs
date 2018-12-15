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

impl Event<'_> {
    pub fn is_pointer_event(&self) -> bool {
        match self {
            Event::PointerButton(..)
            | Event::PointerMotion(..)
            | Event::PointerAbsMotion(..)
            | Event::PointerAxis(..)
            | Event::TouchDown(..)
            | Event::TouchUp(..)
            | Event::TouchMotion(..)
            | Event::TouchCancel(..)
            | Event::TabletAxis(..)
            | Event::TabletProximity(..)
            | Event::TabletTip(..)
            | Event::TabletButton(..) => true,
            Event::Key(..) => false,
        }
    }

    pub fn location(&self) -> Option<Vector2<f64>> {
        match self {
            Event::TouchDown(_, v) | Event::TouchMotion(_, v) => Some(*v),
            _ => None,
        }
    }
}
