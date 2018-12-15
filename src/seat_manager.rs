use crate::server::Server;
use wlroots::*;

pub struct SeatManager {}

impl SeatManager {
    pub fn new() -> SeatManager {
        SeatManager {}
    }
}

impl SeatHandler for SeatManager {
    #[wlroots_dehandle(compositor)]
    fn pointer_grabbed(
        &mut self,
        compositor_handle: CompositorHandle,
        _: SeatHandle,
        _: &PointerGrab,
    ) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.pointer_grabbed = true;
    }

    #[wlroots_dehandle(compositor)]
    fn pointer_released(
        &mut self,
        compositor_handle: CompositorHandle,
        _: SeatHandle,
        _: &PointerGrab,
    ) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.pointer_grabbed = false;
    }

    fn keyboard_grabbed(&mut self, _: CompositorHandle, _: SeatHandle, _: &KeyboardGrab) {
        info!("Keyboard grabbed!");
    }

    fn keyboard_released(&mut self, _: CompositorHandle, _: SeatHandle, _: &KeyboardGrab) {
        info!("Keyboard released!");
    }

    #[wlroots_dehandle(compositor)]
    fn touch_grabbed(&mut self, compositor_handle: CompositorHandle, _: SeatHandle, _: &TouchGrab) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.touch_grabbed = true;
    }

    #[wlroots_dehandle(compositor)]
    fn touch_released(
        &mut self,
        compositor_handle: CompositorHandle,
        _: SeatHandle,
        _: &TouchGrab,
    ) {
        use compositor_handle as compositor;
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.touch_grabbed = false;
    }
}
