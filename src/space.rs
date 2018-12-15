use crate::event::Event;
use crate::renderer::HomeBar;
use crate::renderer::Renderer;
use crate::server::Server;
use crate::view::View;
use cgmath::Matrix4;
use std::mem;
use std::rc::Rc;
use wlroots::utils::current_time;
use wlroots::TouchId;
use wlroots::{wlroots_dehandle, XdgV6ShellSurfaceHandle};

#[derive(Debug)]
pub struct Space {
    views: Vec<Rc<View>>,
    home_bar: Option<HomeBar>,
    home_bar_captured_events: bool,
    pointer_target: Option<usize>,
}

impl Space {
    pub fn new() -> Space {
        Space {
            views: Vec::new(),
            home_bar: None,
            home_bar_captured_events: false,
            pointer_target: None,
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

    #[wlroots_dehandle(seat)]
    pub fn handle_event(&mut self, event: Event, server: &mut Server) {
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

        let mut event_target = None;
        let mut target_index = self.views.len();
        if event.is_pointer_event()
            && ((server.touch_grabbed || server.pointer_grabbed) || event.location().is_none())
        {
            if let Some(target) = self.pointer_target {
                event_target = Some(&self.views[target]);
                target_index = target;
            }
        }

        if event_target.is_none() {
            if let Some(loc) = event.location() {
                for view in self.views.iter().rev() {
                    target_index -= 1;
                    if view.contains_point(loc) {
                        event_target = Some(view);
                        break;
                    }
                }
            }
        }

        if let Some(event_target) = event_target {
            if event.is_pointer_event() {
                if let Some(loc) = event.location() {
                    if Some(target_index) != self.pointer_target {
                        let (sx, sy) = event_target.map_location(loc);
                        event_target.with_surface(|surf| {
                            let seat_h = &server.seat;
                            use seat_h as seat;
                            seat.pointer_notify_enter(surf, sx, sy);
                        });
                        self.pointer_target = Some(target_index);
                    }
                }

                let seat_h = &server.seat;
                use seat_h as seat;

                let time = current_time();

                match event {
                    Event::PointerButton(event) => {
                        seat.pointer_notify_button(time, event.button(), event.state() as u32);
                    }
                    Event::PointerMotion(_) | Event::PointerAbsMotion(_) => {
                        // TODO: seat.pointer_notify_motion(time, sx, sy);
                    }
                    Event::PointerAxis(event) => {
                        // FIXME: where does value_discrete come from?
                        seat.pointer_notify_axis(
                            time,
                            event.orientation(),
                            event.delta(),
                            0,
                            event.source(),
                        );
                    }
                    Event::TouchDown(event, loc) => {
                        let (sx, sy) = event_target.map_location(loc);
                        event_target.with_surface(|surf| {
                            // FIXME: where do TouchIDs come from?
                            let touch_id =
                                unsafe { mem::transmute::<i32, TouchId>(event.touch_id()) };
                            seat.touch_notify_down(surf, time, touch_id, sx, sy);
                        });
                    }
                    Event::TouchMotion(event, loc) => {
                        let (sx, sy) = event_target.map_location(loc);
                        // FIXME: where do TouchIDs come from?
                        let touch_id = unsafe { mem::transmute::<i32, TouchId>(event.touch_id()) };
                        seat.touch_notify_motion(time, touch_id, sx, sy);
                    }
                    Event::TouchUp(event) => {
                        // FIXME: where do TouchIDs come from?
                        let touch_id = unsafe { mem::transmute::<i32, TouchId>(event.touch_id()) };
                        seat.touch_notify_up(time, touch_id);
                    }
                    _ => warn!("Unhandled event"),
                }
            }
        }
    }
}
