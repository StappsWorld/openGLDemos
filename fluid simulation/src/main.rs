extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use gl::types::GLuint;
use glutin_window::GlutinWindow as Window;
use image::Rgba;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop, MouseCursorEvent, RenderArgs};
use rand::Rng;
use Fluid_Simulation::{
    fluid::Fluid, scl, vector::Vector, FPSCounter, WINDOW_HEIGHT, WINDOW_WIDTH,
};

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
const MAGENTA: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
const LIGHT_GRAY: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
const LIGHT_BLUE: [f32; 4] = [0.5, 0.5, 1.0, 1.0];
const LIGHT_GREEN: [f32; 4] = [0.0, 1.0, 0.5, 1.0];
const LIGHT_RED: [f32; 4] = [1.0, 0.0, 0.5, 1.0];

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Fluid Simulation", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut rng = rand::thread_rng();

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(10000);

    let mut fluid = Fluid::new(0, 0.000001, 0.01);

    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);

                fluid.step();
                fluid.renderD(gl, &args);
                fluid.fadeD();
            });
            window.set_title(format!("Fluid Simulation | {:03} fps", fps_counter.tick()));
        } else if let Some([x, y]) = e.mouse_cursor_args() {
            for i in 0..5 {
                let mut v = Vector { x: x, y: y };
                v *= 2.0;
                let x = x / scl as f64 + rng.gen_range(-2..3) as f64;
                let y = y / scl as f64 + rng.gen_range(-2..3) as f64;
                fluid.add_velocity(x.floor() as u32, y.floor() as u32, v.x, v.y);
            }

            for x in x.floor() as u32 - 2..x.floor() as u32 + 2 {
                for y in y.floor() as u32 - 2..y.floor() as u32 + 2 {
                    fluid.add_density(x / scl as u32, y / scl as u32, rng.gen_range(10..25) as f64);
                }
            }
        }
    }
}

fn from_rgba(pack: [f32; 4]) -> [f32; 4] {
    let [r, g, b, a] = pack;
    let [r_f, g_f, b_f] = [r / 255.0, g / 255.0, b / 255.0];
    [r_f, g_f, b_f, a]
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
