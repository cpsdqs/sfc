use crate::server::Server;
use crate::view::View;
use std::rc::Rc;
use wlroots::*;

/// Why does this exist? Because for some reason XdgV6TopLevel.app_id() panics
/// when the app id string is invalid instead of simply returning an error
fn obtain_app_id_safely_unsafely(top_level: &XdgV6TopLevel) -> String {
    unsafe {
        use std::ffi::CStr;
        use std::mem;
        use wlroots::wlroots_sys::{wlr_xdg_surface_v6, wlr_xdg_toplevel_v6};

        // horrible hack that relies on deterministic struct layout
        #[derive(Debug, Eq, PartialEq, Hash)]
        struct HorribleHack {
            _shell_surface: *mut wlr_xdg_surface_v6,
            toplevel: *mut wlr_xdg_toplevel_v6,
        }

        let hh = mem::transmute_copy::<_, HorribleHack>(top_level);

        if (*hh.toplevel).app_id.is_null() {
            return String::from("???");
        }
        let cstr = CStr::from_ptr((*hh.toplevel).app_id);

        match cstr.to_str() {
            Ok(s) => String::from(s),
            Err(_) => String::from("???"),
        }
    }
}

pub struct XdgV6Shell {
    surface: XdgV6ShellSurfaceHandle,
}

impl XdgV6Shell {
    pub fn new(surface: XdgV6ShellSurfaceHandle) -> XdgV6Shell {
        XdgV6Shell { surface }
    }
}

impl XdgV6ShellHandler for XdgV6Shell {
    #[wlroots_dehandle(compositor, surface)]
    fn map_request(
        &mut self,
        compositor_handle: CompositorHandle,
        surface_handle: SurfaceHandle,
        xdg_surface_handle: XdgV6ShellSurfaceHandle,
    ) {
        use compositor_handle as compositor;

        let app_id = {
            use xdg_surface_handle as surface;

            match surface.state().unwrap() {
                XdgV6ShellState::TopLevel(top_level) => {
                    Some(obtain_app_id_safely_unsafely(top_level))
                }
                _ => None,
            }
        };

        if let Some(app_id) = app_id {
            let server: &mut Server = compositor.data.downcast_mut().unwrap();

            let view = Rc::new(View::new(xdg_surface_handle));
            server.add_view(app_id, view);
        }
    }

    #[wlroots_dehandle(compositor)]
    fn unmap_request(
        &mut self,
        compositor_handle: CompositorHandle,
        _: SurfaceHandle,
        surface: XdgV6ShellSurfaceHandle,
    ) {
        use compositor_handle as compositor;

        let server: &mut Server = compositor.data.downcast_mut().unwrap();

        server.remove_view_for_surface(surface);
    }
}

pub struct XdgV6ShellManager;

impl XdgV6ShellManager {
    pub fn new() -> XdgV6ShellManager {
        XdgV6ShellManager
    }
}

impl XdgV6ShellManagerHandler for XdgV6ShellManager {
    fn new_surface(
        &mut self,
        _: CompositorHandle,
        surface: XdgV6ShellSurfaceHandle,
    ) -> (Option<Box<XdgV6ShellHandler>>, Option<Box<SurfaceHandler>>) {
        (Some(Box::new(XdgV6Shell::new(surface))), None)
    }
}
