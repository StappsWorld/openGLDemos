extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use ::image::Rgba;
use gl::types::GLuint;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop, MouseCursorEvent, RenderArgs};
use rand::Rng;
use vector_refection::{particle::Particle, FPSCounter, *};
use yalal::{line::Line, matrix::Matrix, vector::*};

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window =
        WindowSettings::new("Vector Reflection Test", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut rng = rand::thread_rng();

    let wall = Line::new(
        Vector::new(200.0, 200.0),
        Vector::new(200.0, WINDOW_HEIGHT as f64 - 200.0),
    );

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(10000);
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
                line(
                    WHITE,
                    1.0,
                    {
                        let p1 = wall.p1();
                        let p2 = wall.p2();
                        [
                            p1.x_y().0,
                            p1.x_y().1,
                            p2.x_y().0,
                            p2.x_y().1,
                        ]
                    },
                    c.transform,
                    gl,
                );
                let midpoint = wall.midpoint();
                let (midpoint_x, midpoint_y) = midpoint.x_y();
                let normals_raw = wall.get_normals();
                let mut right = normals_raw.0.clone() + midpoint.clone();
                right.normalize();
                right *= 100.0;
                let mut left = normals_raw.1.clone() + midpoint.clone();
                left.normalize();
                left *= -100.0;
                ellipse(
                    RED,
                    [-4.0, -4.0, 8.0, 8.0],
                    c.transform.trans(midpoint_x, midpoint_y),
                    gl,
                );
                let midpoint_trans = c.transform.trans(midpoint_x, midpoint_y);
                line(
                    CYAN,
                    1.0,
                    [
                        0.0,
                        0.0,
                        right.x_y().0,
                        right.x_y().1,
                    ],
                    midpoint_trans,
                    gl,
                );
                line(
                    BLUE,
                    1.0,
                    [
                        0.0,
                        0.0,
                        left.x_y().0,
                        left.x_y().1,
                    ],
                    midpoint_trans,
                    gl,
                );
            });
            window.set_title(format!(
                "Vector Reflection Test | {:03} fps",
                fps_counter.tick()
            ));
        }
    }
}
