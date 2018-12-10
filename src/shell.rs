use crate::server::Server;
use crate::view::View;
use std::rc::Rc;
use wlroots::*;

pub struct XdgV6Shell {
    surface: XdgV6ShellSurfaceHandle,
}

impl XdgV6Shell {
    pub fn new(surface: XdgV6ShellSurfaceHandle) -> XdgV6Shell {
        XdgV6Shell { surface }
    }
}

impl XdgV6ShellHandler for XdgV6Shell {
    #[wlroots_dehandle(compositor)]
    fn map_request(
        &mut self,
        compositor_handle: CompositorHandle,
        _: SurfaceHandle,
        surface: XdgV6ShellSurfaceHandle,
    ) {
        use compositor_handle as compositor;

        let is_toplevel = with_handles!([(shell_surface: {&surface})] => {
            match shell_surface.state().unwrap() {
                XdgV6ShellState::TopLevel(_) => true,
                _ => false,
            }
        })
        .unwrap();

        let server: &mut Server = compositor.data.downcast_mut().unwrap();

        if is_toplevel {
            let view = Rc::new(View::new(surface));
            server.views.push(view);
        };
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

        if let Some(pos) = server.views.iter().position(|x| x.shell == surface) {
            server.views.remove(pos);
        } else {
            error!("Received an unmap request but the surface couldn’t be found");
        };
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
