use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::window::WindowSettings;
use piston::PressEvent;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};


fn main() {
    // Change this to OpenGL::V2_1 if not working.
	let opengl = OpenGL::V3_2;

	// Create a Glutin window.
	let mut window: Window = WindowSettings::new("bird", [300, 388])
	.graphics_api(opengl)
	.exit_on_esc(true)
	.build()
	.unwrap();


	let mut events = Events::new(EventSettings::new());
	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			;//app.render(&args);
		}
		if let Some(args) = e.update_args() {
			;//app.update(&args);
		}
		if let Some(button) = e.press_args() {
			;//app.press(&button);
		}
	}
}
