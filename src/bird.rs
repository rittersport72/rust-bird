use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use piston::Button;

// Window dimensions
pub const WIDTH: f64 = 300.0;
pub const HEIGHT: f64 = 388.0;

const BACKGROUND_COLOR: [f32; 4] = [0.14, 0.36, 0.46, 1.0]; // blue
const BIRD_COLOR: [f32; 4] = [0.87, 0.84, 0.27, 1.0]; // yellow

// Pipe
const PIPE_WIDTH: f64 = 50.0;
const PIPE_SPACE_HEIGHT: f64 = 100.0;
const PIPE_COLOR: [f32; 4] = [0.37, 0.82, 0.28, 1.0]; // green

pub struct Application {
    gl: GlGraphics,
    bird_y_posit: f64,
    bird_y_speed: f64,
    pipe1_x_posit: f64,
    pipe2_x_posit: f64,
    pipe1_space_y: f64,
    pipe2_space_y: f64,
}

impl Application {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        let app = Application {
            gl: GlGraphics::new(opengl),
            bird_y_posit: 200.0,
            bird_y_speed: 0.0,
            pipe1_x_posit: WIDTH,
            pipe2_x_posit: WIDTH + (WIDTH + PIPE_WIDTH) / 2.0,
            pipe1_space_y: Application::random_pipe_space_y(),
            pipe2_space_y: Application::random_pipe_space_y(),
        };
        app
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND_COLOR, gl);

            // Draw bird
            let rect = [62.0, self.bird_y_posit, 30.0, 25.0];
            rectangle(BIRD_COLOR, rect, c.transform, gl);

            // Draw upper pipe 1
            let upper_pipe = [self.pipe1_x_posit, 0.0, PIPE_WIDTH, self.pipe1_space_y];
            rectangle(PIPE_COLOR, upper_pipe, c.transform, gl);

            // Draw lower pipe 1
            let lower_pipe = [self.pipe1_x_posit, self.pipe1_space_y + PIPE_SPACE_HEIGHT,
                PIPE_WIDTH, HEIGHT - self.pipe1_space_y - PIPE_SPACE_HEIGHT];
            rectangle(PIPE_COLOR, lower_pipe, c.transform, gl);

            // Draw upper pipe 2
            let upper_pipe = [self.pipe2_x_posit, 0.0, PIPE_WIDTH, self.pipe2_space_y];
            rectangle(PIPE_COLOR, upper_pipe, c.transform, gl);

            // Draw lower pipe 2
            let lower_pipe = [self.pipe2_x_posit, self.pipe2_space_y + PIPE_SPACE_HEIGHT,
                PIPE_WIDTH, HEIGHT - self.pipe2_space_y - PIPE_SPACE_HEIGHT];
            rectangle(PIPE_COLOR, lower_pipe, c.transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.bird_y_speed += 500.0 * args.dt;
        self.bird_y_posit += self.bird_y_speed * args.dt;

        if (self.bird_y_posit - 10.0) > HEIGHT {
            self.bird_y_posit = HEIGHT - 10.0;
            self.bird_y_speed = 0.0;
        }

        self.pipe1_x_posit = self.pipe1_x_posit - (60.0 * args.dt);
        self.pipe2_x_posit = self.pipe2_x_posit - (60.0 * args.dt);

        if (self.pipe1_x_posit + PIPE_WIDTH) < 0.0 {
            self.pipe1_x_posit = WIDTH;
            self.pipe1_space_y = Application::random_pipe_space_y();
        }

        if (self.pipe2_x_posit + PIPE_WIDTH) < 0.0 {
            self.pipe2_x_posit = WIDTH;
            self.pipe2_space_y = Application::random_pipe_space_y();
        }
    }

    pub fn press(&mut self, _button: &Button) {
        if self.bird_y_posit > 0.0 {
            self.bird_y_speed = -165.0;
        }
    }

    fn random_pipe_space_y() -> f64 {
        // TODO: random
        150.0
    }
}
