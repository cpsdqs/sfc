use crate::event::Event;
use crate::server::Server;
use std::process::Command;
use wlroots::key_events::*;
use wlroots::pointer_events::*;
use wlroots::tablet_tool_events::{
    AxisEvent as TabletAxisEvent, ButtonEvent as TabletButtonEvent, ProximityEvent, TipEvent,
};
use wlroots::touch_events::*;
use wlroots::*;

#[derive(Debug)]
pub struct InputManager;

impl InputManager {
    pub fn new() -> InputManager {
        InputManager
    }
}

impl InputManagerHandler for InputManager {
    fn input_added(&mut self, _: CompositorHandle, dev: &mut InputDevice) {
        info!("Input added! {:?}", dev.dev_type());
    }

    #[wlroots_dehandle(compositor)]
    fn keyboard_added(
        &mut self,
        compositor_handle: CompositorHandle,
        _: KeyboardHandle,
    ) -> Option<Box<KeyboardHandler>> {
        info!("Keyboard added!");

        {
            use compositor_handle as compositor;
            let server: &mut Server = compositor.data.downcast_mut().unwrap();
            server.keyboard_added();
        }

        Some(Box::new(SfKeyboardHandler {
            ctrl_alt_pressed: false,
            ctrl_shift_opt_pressed: false,
        }))
    }

    #[wlroots_dehandle(compositor)]
    fn pointer_added(
        &mut self,
        compositor_handle: CompositorHandle,
        _: PointerHandle,
    ) -> Option<Box<PointerHandler>> {
        info!("Pointer added!");

        {
            use compositor_handle as compositor;
            let server: &mut Server = compositor.data.downcast_mut().unwrap();
            server.pointer_added();
        }

        Some(Box::new(SfPointerHandler))
    }

    #[wlroots_dehandle(compositor)]
    fn touch_added(
        &mut self,
        compositor_handle: CompositorHandle,
        _: TouchHandle,
    ) -> Option<Box<TouchHandler>> {
        info!("Touch added!");

        {
            use compositor_handle as compositor;
            let server: &mut Server = compositor.data.downcast_mut().unwrap();
            server.touch_added();
        }

        Some(Box::new(SfTouchHandler))
    }

    fn tablet_tool_added(
        &mut self,
        _: CompositorHandle,
        _: TabletToolHandle,
    ) -> Option<Box<TabletToolHandler>> {
        info!("Tablet tool added!");
        Some(Box::new(SfTabletToolHandler))
    }

    fn tablet_pad_added(
        &mut self,
        _: CompositorHandle,
        _: TabletPadHandle,
    ) -> Option<Box<TabletPadHandler>> {
        info!("Tablet pad added!");
        None
    }
}

pub struct SfPointerHandler;

impl PointerHandler for SfPointerHandler {
    #[wlroots_dehandle(compositor)]
    fn on_button(
        &mut self,
        compositor_handle: CompositorHandle,
        _: PointerHandle,
        event: &ButtonEvent,
    ) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.handle_event(Event::PointerButton(event));
    }
    #[wlroots_dehandle(compositor)]
    fn on_motion(
        &mut self,
        compositor_handle: CompositorHandle,
        _: PointerHandle,
        event: &pointer_events::MotionEvent,
    ) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.handle_event(Event::PointerMotion(event));
    }
    #[wlroots_dehandle(compositor)]
    fn on_motion_absolute(
        &mut self,
        compositor_handle: CompositorHandle,
        _: PointerHandle,
        event: &pointer_events::AbsoluteMotionEvent,
    ) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.handle_event(Event::PointerAbsMotion(event));
    }
    #[wlroots_dehandle(compositor)]
    fn on_axis(
        &mut self,
        compositor_handle: CompositorHandle,
        _: PointerHandle,
        event: &AxisEvent,
    ) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.handle_event(Event::PointerAxis(event));
    }
}

pub struct SfKeyboardHandler {
    ctrl_alt_pressed: bool,
    ctrl_shift_opt_pressed: bool,
}

impl KeyboardHandler for SfKeyboardHandler {
    #[wlroots_dehandle(keyboard)]
    fn modifiers(&mut self, _: CompositorHandle, keyboard_handle: KeyboardHandle) {
        use keyboard_handle as keyboard;
        let mods = keyboard.get_modifiers();
        let opt_pressed = mods.contains(KeyboardModifier::WLR_MODIFIER_ALT);
        let ctrl_pressed = mods.contains(KeyboardModifier::WLR_MODIFIER_CTRL);
        let shift_pressed = mods.contains(KeyboardModifier::WLR_MODIFIER_SHIFT);

        self.ctrl_alt_pressed = opt_pressed && ctrl_pressed;
        self.ctrl_shift_opt_pressed = opt_pressed && ctrl_pressed && shift_pressed;
    }

    #[wlroots_dehandle(compositor)]
    fn on_key(&mut self, compositor_handle: CompositorHandle, _: KeyboardHandle, event: &KeyEvent) {
        use compositor_handle as compositor;

        // F-keys
        if self.ctrl_alt_pressed && event.keycode() >= 59 && event.keycode() < 59 + 7 {
            // this is probably the wrong way to do this but it works
            Command::new("chvt")
                .arg(format!("{}", event.keycode() - 58))
                .output()
                .expect("Failed to run chvt");
        }

        if self.ctrl_shift_opt_pressed && event.keycode() == 1 {
            // quit immediately
            std::process::exit(0);
        }

        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.handle_event(Event::Key(event));
    }
}

pub struct SfTouchHandler;

impl TouchHandler for SfTouchHandler {
    #[wlroots_dehandle(compositor)]
    fn on_down(&mut self, compositor_handle: CompositorHandle, _: TouchHandle, event: &DownEvent) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();

        let location = event.location();
        server.handle_event(Event::TouchDown(event, location.into()));
    }

    #[wlroots_dehandle(compositor)]
    fn on_up(&mut self, compositor_handle: CompositorHandle, _: TouchHandle, event: &UpEvent) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.handle_event(Event::TouchUp(event));
    }

    #[wlroots_dehandle(compositor)]
    fn on_motion(
        &mut self,
        compositor_handle: CompositorHandle,
        _: TouchHandle,
        event: &touch_events::MotionEvent,
    ) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        let location = event.location();
        server.handle_event(Event::TouchMotion(event, location.into()));
    }

    #[wlroots_dehandle(compositor)]
    fn on_cancel(
        &mut self,
        compositor_handle: CompositorHandle,
        _: TouchHandle,
        event: &CancelEvent,
    ) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.handle_event(Event::TouchCancel(event));
    }
}

pub struct SfTabletToolHandler;

impl TabletToolHandler for SfTabletToolHandler {
    fn on_axis(&mut self, _: CompositorHandle, _: TabletToolHandle, _: &TabletAxisEvent) {
        println!("axis");
    }

    fn on_proximity(&mut self, _: CompositorHandle, _: TabletToolHandle, _: &ProximityEvent) {
        println!("proximity");
    }

    fn on_tip(&mut self, _: CompositorHandle, _: TabletToolHandle, _: &TipEvent) {
        println!("tip");
    }

    fn on_button(&mut self, _: CompositorHandle, _: TabletToolHandle, _: &TabletButtonEvent) {
        println!("tablet button");
    }

    fn destroyed(&mut self, _: CompositorHandle, _: TabletToolHandle) {
        println!("tablet destroyed");
    }
}
