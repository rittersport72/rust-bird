use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use piston::Button;

// Window dimensions
pub const WIDTH:f64 = 300.0;
pub const HEIGHT:f64 = 388.0;

pub struct Application {
    gl: GlGraphics,
    bird_color: [f32; 4],
    background_color: [f32; 4],
    bird_y_posit: f64,
    bird_y_speed: f64,
}

impl Application {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        let app = Application {
            gl: GlGraphics::new(opengl),
            bird_color: [0.87, 0.84, 0.27, 1.0], // yellow
            background_color: [0.14, 0.36, 0.46, 1.0], // blue
            bird_y_posit: 200.0,
            bird_y_speed: 0.0,
        };
        app
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(self.background_color, gl);

            let rect = [62.0, self.bird_y_posit, 30.0, 25.0];
            // Draw bird
            rectangle(self.bird_color, rect, c.transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.bird_y_speed += 500.0 * args.dt;
        self.bird_y_posit += self.bird_y_speed * args.dt;
    }

    pub fn press(&mut self, _button: &Button) {
        if self.bird_y_posit > 0.0 {
            self.bird_y_speed = -165.0;
        }
    }
}
