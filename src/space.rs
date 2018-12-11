use wlroots::XdgV6ShellSurfaceHandle;
use crate::server::Server;
use crate::event::Event;
use crate::renderer::HomeBar;
use crate::renderer::Renderer;
use crate::view::View;
use cgmath::Matrix4;
use std::rc::Rc;

#[derive(Debug)]
pub struct Space {
    views: Vec<Rc<View>>,
    home_bar: Option<HomeBar>,
    home_bar_captured_events: bool,
}

impl Space {
    pub fn new() -> Space {
        Space {
            views: Vec::new(),
            home_bar: None,
            home_bar_captured_events: false,
        }
    }

    pub fn add_view(&mut self, view: Rc<View>) {
        self.views.push(view);
    }

    pub fn remove_view_for_surface(&mut self, surface: &XdgV6ShellSurfaceHandle) -> bool {
        if let Some(pos) = self.views.iter().position(|x| &x.shell == surface) {
            self.views.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn render(
        &mut self,
        matrix: Matrix4<f32>,
        renderer: &Renderer,
        wlr_renderer: &mut wlroots::Renderer,
    ) {
        if self.home_bar.is_none() {
            let (width, height) = renderer.dimensions();
            let resolution = renderer.resolution();
            self.home_bar = Some(HomeBar::new(width, height, resolution));
        }

        for view in self.views.iter_mut().rev() {
            view.render(matrix, wlr_renderer);
        }

        self.home_bar.as_mut().unwrap().render(matrix);
    }

    pub fn handle_event(&mut self, event: Event, _server: &mut Server) {
        // TODO: send events to windows, too
        if let Some(ref mut home_bar) = self.home_bar {
            match event {
                Event::TouchDown(_, pos) => {
                    if home_bar.should_capture_event(pos.x, pos.y) {
                        self.home_bar_captured_events = true;
                        home_bar.handle_event(event);
                        return;
                    }
                }
                Event::TouchMotion(..) => {
                    if self.home_bar_captured_events {
                        home_bar.handle_event(event);
                        return;
                    }
                }
                Event::TouchUp(..) => {
                    if self.home_bar_captured_events {
                        home_bar.handle_event(event);
                        self.home_bar_captured_events = false;
                        return;
                    }
                }
                _ => (),
            }
        }
    }
}
