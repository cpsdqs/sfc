use crate::renderer::CairoTex;
use crate::renderer::Renderer;
use cgmath::Matrix4;

struct DockItem {
    command: String,
}

pub struct Dock {
    items: Vec<DockItem>,
    cairo_tex: Option<CairoTex>,
}

impl Dock {
    pub fn new() -> Dock {
        Dock {
            items: vec![
                DockItem {
                    command: "weston-terminal".into(),
                },
                DockItem {
                    command: "weston-simple-shm".into(),
                },
            ],
            cairo_tex: None,
        }
    }

    fn draw(&mut self, renderer: &Renderer) {
        if self.cairo_tex.is_none() {
            let (width, height) = renderer.dimensions();
            let resolution = renderer.resolution();
            self.cairo_tex = Some(CairoTex::new(width, height, resolution));
        }

        let cairo_tex = self.cairo_tex.as_ref().unwrap();
        cairo_tex.clear();
    }

    pub fn render(&mut self, matrix: Matrix4<f32>, renderer: &Renderer) {
        self.draw(renderer);
    }
}
