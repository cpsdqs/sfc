use wlroots::XdgV6ShellSurfaceHandle;
use std::rc::Rc;
use crate::view::View;
use crate::space::Space;
use crate::event::Event;
use crate::renderer::Renderer;
use wlroots::SeatHandle;
use std::collections::HashMap;

pub type SpaceID = u64;

#[derive(Debug)]
pub struct Server {
    spaces: HashMap<SpaceID, Space>,
    space_order: Vec<SpaceID>,
    space_id_counter: SpaceID,
    app_id_mapping: HashMap<String, SpaceID>,
    pub seat: SeatHandle,
    pub renderer: Option<Renderer>,
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
        }
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

    pub fn handle_event(&mut self, event: Event) {
        if self.renderer.is_none() {
            warn!("No renderer, ignoring event");
        }
        let mut renderer = self.renderer.take().unwrap();
        renderer.handle_event(event, self);
        self.renderer = Some(renderer);
    }
}
