use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use piston::Button;

pub struct Application {
    gl: GlGraphics,
    fg: [f32; 4],
    bg: [f32; 4],
}

impl Application {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        let mut app = Application {
            gl: GlGraphics::new(opengl),
            fg: [0.87, 0.84, 0.27, 1.0], // yellow
            bg: [0.14, 0.36, 0.46, 1.0], // blue
        };
        app
    }

    pub fn render(&mut self, args: &RenderArgs) {}

    pub fn update(&mut self, args: &UpdateArgs) {}

    pub fn press(&mut self, button: &Button) {}
}
