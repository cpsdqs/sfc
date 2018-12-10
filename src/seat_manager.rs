use wlroots::*;

pub struct SeatManager {}

impl SeatManager {
    pub fn new() -> SeatManager {
        SeatManager {}
    }
}

impl SeatHandler for SeatManager {
    fn pointer_grabbed(&mut self, _: CompositorHandle, _: SeatHandle, _: &PointerGrab) {
        info!("Pointer grabbed!");
    }

    fn pointer_released(&mut self, _: CompositorHandle, _: SeatHandle, _: &PointerGrab) {
        info!("Pointer released!");
    }

    fn keyboard_grabbed(&mut self, _: CompositorHandle, _: SeatHandle, _: &KeyboardGrab) {
        info!("Keyboard grabbed!");
    }

    fn keyboard_released(&mut self, _: CompositorHandle, _: SeatHandle, _: &KeyboardGrab) {
        info!("Keyboard released!");
    }

    fn touch_grabbed(&mut self, _: CompositorHandle, _: SeatHandle, _: &TouchGrab) {
        info!("Touch grabbed!");
    }

    fn touch_released(&mut self, _: CompositorHandle, _: SeatHandle, _: &TouchGrab) {
        info!("Touch released!");
    }
}
