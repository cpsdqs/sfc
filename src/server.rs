use crate::event::{Event, RawEvent};
use crate::renderer::Renderer;
use crate::space::Space;
use crate::view::View;
use std::collections::HashMap;
use std::rc::Rc;
use wlroots::{wlroots_dehandle, Capability, SeatHandle, XdgV6ShellSurfaceHandle};

pub type SpaceID = u64;

#[derive(Debug)]
pub struct Server {
    spaces: HashMap<SpaceID, Space>,
    space_order: Vec<SpaceID>,
    space_id_counter: SpaceID,
    app_id_mapping: HashMap<String, SpaceID>,
    pub seat: SeatHandle,
    pub renderer: Option<Renderer>,
    keyboards: usize,
    pointers: usize,
    touch: usize,
    pub pointer_grabbed: bool,
    pub touch_grabbed: bool,
}

impl Server {
    pub fn new() -> Server {
        Server {
            spaces: HashMap::new(),
            space_order: Vec::new(),
            space_id_counter: 0,
            app_id_mapping: HashMap::new(),
            seat: SeatHandle::default(),
            renderer: None,
            keyboards: 0,
            pointers: 0,
            touch: 0,
            pointer_grabbed: false,
            touch_grabbed: false,
        }
    }

    #[wlroots_dehandle(seat)]
    fn update_capabilities(&mut self) {
        let mut caps = Capability::empty();
        if self.keyboards > 0 {
            caps |= Capability::Keyboard;
        }
        if self.pointers > 0 {
            caps |= Capability::Pointer;
        }
        if self.touch > 0 {
            caps |= Capability::Touch;
        }

        let seat_h = &self.seat;
        use seat_h as seat;
        seat.set_capabilities(caps);
    }

    pub fn keyboard_added(&mut self) {
        self.keyboards += 1;
        self.update_capabilities();
    }

    pub fn keyboard_removed(&mut self) {
        self.keyboards -= 1;
        self.update_capabilities();
    }

    pub fn pointer_added(&mut self) {
        self.pointers += 1;
        self.update_capabilities();
    }

    pub fn pointer_removed(&mut self) {
        self.pointers -= 1;
        self.update_capabilities();
    }

    pub fn touch_added(&mut self) {
        self.touch += 1;
        self.update_capabilities();
    }

    pub fn touch_removed(&mut self) {
        self.touch -= 1;
        self.update_capabilities();
    }

    fn add_space(&mut self) -> SpaceID {
        let id = self.space_id_counter;
        self.space_id_counter += 1;
        self.spaces.insert(id, Space::new());
        self.space_order.push(id);
        id
    }

    pub fn add_view(&mut self, app_id: String, view: Rc<View>) {
        let space_id = if !self.app_id_mapping.contains_key(&app_id) {
            let space_id = self.add_space();
            self.app_id_mapping.insert(app_id, space_id);
            space_id
        } else {
            *self.app_id_mapping.get(&app_id).unwrap()
        };

        self.spaces.get_mut(&space_id).unwrap().add_view(view);
    }

    pub fn remove_view_for_surface(&mut self, surface: XdgV6ShellSurfaceHandle) {
        for (_, space) in &mut self.spaces {
            let removed = space.remove_view_for_surface(&surface);
            if removed {
                break;
            }
        }
    }

    pub fn space(&self, id: SpaceID) -> Option<&Space> {
        self.spaces.get(&id)
    }

    pub fn space_mut(&mut self, id: SpaceID) -> Option<&mut Space> {
        self.spaces.get_mut(&id)
    }

    pub fn space_order(&self) -> &[SpaceID] {
        &*self.space_order
    }

    pub fn handle_event(&mut self, event: RawEvent) {
        if self.renderer.is_none() {
            warn!("No renderer, ignoring event");
        }

        let renderer = self.renderer.take().unwrap();
        let mut event = event.into();
        renderer.map_event(&mut event);
        self.renderer = Some(renderer);

        if let Some(top_space_id) = self.space_order.last().map(|x| *x) {
            let mut space = self.spaces.remove(&top_space_id).unwrap();
            space.handle_event(event, self);
            self.spaces.insert(top_space_id, space);
        }
    }
}
