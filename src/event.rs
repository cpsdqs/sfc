use cgmath::Vector2;
use wlroots::key_events as key;
use wlroots::pointer_events as pointer;
use wlroots::tablet_tool_events as tool;
use wlroots::touch_events as touch;
use wlroots::wlr_axis_orientation;
use wlroots::wlr_axis_source;
use wlroots::TouchId;
use wlroots::wlr_tablet_tool_proximity_state::*;
use wlroots::wlr_key_state::*;
use std::mem;

pub enum Event {
    PointerDown {
        button: u32,
    },
    PointerUp {
        button: u32,
    },
    PointerMotion {
        location: Vector2<f64>,
    },
    PointerAxis {
        source: wlr_axis_source,
        orientation: wlr_axis_orientation,
        delta: f64,
    },
    TouchDown {
        id: TouchId,
        location: Vector2<f64>,
    },
    TouchMotion {
        id: TouchId,
        location: Vector2<f64>,
    },
    TouchUp {
        id: TouchId,
    },
    TouchCancel {
        id: TouchId,
    },
    TabletProximityIn {
        location: Vector2<f64>,
    },
    TabletProximityOut {
        location: Vector2<f64>,
    },
    TabletTipDown {
        location: Vector2<f64>,
    },
    TabletTipUp {
        location: Vector2<f64>,
    },
    TabletButtonDown {
        button: u32,
    },
    TabletButtonUp {
        button: u32,
    },
    TabletAxis {
        location: Vector2<f64>,
        pressure: f64,
        distance: f64,
        tilt: (f64, f64),
        slider: f64,
        wheel_delta: f64,
    },
    KeyDown {
        code: u32,
    },
    KeyUp {
        code: u32,
    },
}
impl Event {
    pub fn is_pointer_event(&self) -> bool {
        match self {
            | Event::PointerDown { .. }
            | Event::PointerUp { .. }
            | Event::PointerMotion { .. }
            | Event::PointerAxis { .. }
            | Event::TouchDown { .. }
            | Event::TouchMotion { .. }
            | Event::TouchUp { .. }
            | Event::TouchCancel { .. }
            | Event::TabletProximityIn { .. }
            | Event::TabletProximityOut { .. }
            | Event::TabletTipDown { .. }
            | Event::TabletTipUp { .. }
            | Event::TabletButtonDown { .. }
            | Event::TabletButtonUp { .. }
            | Event::TabletAxis { .. } => true,
            | Event::KeyDown { .. }
            | Event::KeyUp { .. } => false,
        }
    }

    // TODO: deduplicate the following two

    pub fn location_mut(&mut self) -> Option<&mut Vector2<f64>> {
        match self {
            | Event::PointerMotion { location, .. }
            | Event::TouchDown { location, .. }
            | Event::TouchMotion { location, .. }
            | Event::TabletProximityIn { location, .. }
            | Event::TabletProximityOut { location, .. }
            | Event::TabletTipDown { location, .. }
            | Event::TabletTipUp { location, .. }
            | Event::TabletAxis { location, .. } => Some(location),
            | Event::PointerDown { .. }
            | Event::PointerUp { .. }
            | Event::PointerAxis { .. }
            | Event::TouchUp { .. }
            | Event::TouchCancel { .. }
            | Event::TabletButtonDown { .. }
            | Event::TabletButtonUp { .. }
            | Event::KeyDown { .. }
            | Event::KeyUp { .. } => None,
        }
    }

    pub fn location(&self) -> Option<Vector2<f64>> {
        match self {
            | Event::PointerMotion { location, .. }
            | Event::TouchDown { location, .. }
            | Event::TouchMotion { location, .. }
            | Event::TabletProximityIn { location, .. }
            | Event::TabletProximityOut { location, .. }
            | Event::TabletTipDown { location, .. }
            | Event::TabletTipUp { location, .. }
            | Event::TabletAxis { location, .. } => Some(*location),
            | Event::PointerDown { .. }
            | Event::PointerUp { .. }
            | Event::PointerAxis { .. }
            | Event::TouchUp { .. }
            | Event::TouchCancel { .. }
            | Event::TabletButtonDown { .. }
            | Event::TabletButtonUp { .. }
            | Event::KeyDown { .. }
            | Event::KeyUp { .. } => None,
        }
    }
}

// I don’t know why the API says that wlroots returns this type, because it doesn’t
fn create_touchid(id: i32) -> TouchId {
    unsafe { mem::transmute::<i32, TouchId>(id) }
}


impl<'a> From<RawEvent<'a>> for Event {
    fn from(event: RawEvent<'a>) -> Event {
        match event {
            RawEvent::TouchDown(event) => Event::TouchDown {
                id: create_touchid(event.touch_id()),
                location: event.location().into(),
            },
            RawEvent::TouchMotion(event) => Event::TouchMotion {
                id: create_touchid(event.touch_id()),
                location: event.location().into(),
            },
            RawEvent::TouchUp(event) => Event::TouchUp {
                id: create_touchid(event.touch_id()),
            },
            RawEvent::TabletProximity(event) => match event.state() {
                WLR_TABLET_TOOL_PROXIMITY_IN => Event::TabletProximityIn {
                    location: event.position().into(),
                },
                WLR_TABLET_TOOL_PROXIMITY_OUT => Event::TabletProximityOut {
                    location: event.position().into(),
                },
            },
            RawEvent::TabletAxis(event) => Event::TabletAxis {
                location: event.position().into(),
                pressure: event.pressure(),
                distance: event.distance(),
                tilt: event.tilt(),
                slider: event.slider(),
                wheel_delta: event.wheel_delta(),
            },
            RawEvent::Key(event) => match event.key_state() {
                WLR_KEY_PRESSED => Event::KeyDown {
                    code: event.keycode(),
                },
                WLR_KEY_RELEASED => Event::KeyUp {
                    code: event.keycode(),
                }
            }
            _ => unimplemented!(),
        }
    }
}

pub enum RawEvent<'a> {
    PointerButton(&'a pointer::ButtonEvent),
    PointerMotion(&'a pointer::MotionEvent),
    PointerAbsMotion(&'a pointer::AbsoluteMotionEvent),
    PointerAxis(&'a pointer::AxisEvent),
    Key(&'a key::KeyEvent),
    TouchDown(&'a touch::DownEvent),
    TouchUp(&'a touch::UpEvent),
    TouchMotion(&'a touch::MotionEvent),
    TouchCancel(&'a touch::CancelEvent),
    TabletAxis(&'a tool::AxisEvent),
    TabletProximity(&'a tool::ProximityEvent),
    TabletTip(&'a tool::TipEvent),
    TabletButton(&'a tool::ButtonEvent),
}
