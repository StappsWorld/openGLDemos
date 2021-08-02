extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use ::image::Rgba;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop, MouseCursorEvent, RenderArgs};
use rand::Rng;
use Autonomous_Agent::{
    vector::Vector, vehicle::Vehicle, FPSCounter, BLACK, WHITE, WINDOW_HEIGHT, WINDOW_WIDTH,
};

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window =
        WindowSettings::new("Autonomous Agent Test", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut rng = rand::thread_rng();

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(10000);

    let mut persuer: Vehicle = Vehicle::new(100.0, 100.0, [0.0, 1.0, 0.0, 1.0]);
    let mut target: Vehicle = Vehicle::new(200.0, 200.0, [1.0, 0.0, 0.0, 1.0]);

    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);


                persuer.update();
                persuer.show(gl, &args);
                persuer.apply_force(persuer.arrive(target.pos));
                persuer.edges();

                target.update();
                target.show(gl, &args);
                //target.apply_force(target.evade(&persuer));
                target.edges();
            });
            window.set_title(format!(
                "Autonomous Agent Test | {:03} fps",
                fps_counter.tick()
            ));
        } else if let Some([x, y]) = e.mouse_cursor_args() {
            // target.pos.x = x;
            // target.pos.y = y;
        }
    }
}
