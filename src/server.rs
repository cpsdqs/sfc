use crate::event::Event;
use crate::renderer::Renderer;
use crate::view::View;
use std::rc::Rc;
use wlroots::SeatHandle;

#[derive(Debug)]
pub struct Server {
    pub views: Vec<Rc<View>>,
    pub seat: SeatHandle,
    pub renderer: Option<Renderer>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            views: Vec::new(),
            seat: SeatHandle::default(),
            renderer: None,
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        if self.renderer.is_none() {
            warn!("No renderer, ignoring event");
        }
        let mut renderer = self.renderer.take().unwrap();
        renderer.handle_event(event, self);
        self.renderer = Some(renderer);
    }
}
