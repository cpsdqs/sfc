use wlroots::*;

#[derive(Debug)]
pub struct View {
    pub shell: XdgV6ShellSurfaceHandle,
}

impl View {
    pub fn new(shell: XdgV6ShellSurfaceHandle) -> View {
        View { shell }
    }

    #[wlroots_dehandle(surface)]
    pub fn for_each_surface(&self, f: &mut FnMut(SurfaceHandle, i32, i32)) {
        let shell = &self.shell;
        use shell as surface;

        surface.for_each_surface(f);
    }
}
