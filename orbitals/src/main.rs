extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use ::image::Rgba;
use gl::types::GLuint;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use orbitals::{particle::Particle, vector::Vector, FPSCounter, *};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop, MouseCursorEvent, RenderArgs};
use rand::Rng;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Orbital", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut rng = rand::thread_rng();

    let mut mouseX = 0.0;
    let mut mouseY = 0.0;

    let mut bodies: Vec<Body> = vec![];

    const max_bodies: u32 = 5;

    for _ in 0..max_bodies {
        bodies.push(Body::random());
    }

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(300);
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);

                let mut len = bodies.len();
                let mut old = bodies.clone();
                let mut i = 0;
                while i < len {
                    let body = &mut bodies[i];
                    let mut j = 0;
                    'inner: while j < len {
                        if i != j {
                            let other = &old[j];
                            body.pull(other);
                            body.particle.update();
                            body.particle.edges();

                            if body.inside(other) {
                                //println!("Body {} was inside body {}! Distance of {} and smallest distance between is {}", i, j, body.particle.pos.distance(&other.particle.pos), other.radius + body.radius);
                                bodies.remove(i);
                                i -= 1;
                                len -= 1;
                                old = bodies.clone();
                                break 'inner;
                            }
                        }
                        j += 1;
                    }
                    i += 1;
                }

                for body in bodies.clone() {
                    let rad = body.radius;
                    ellipse(
                        body.particle.color,
                        [
                            body.particle.pos.x - rad,
                            body.particle.pos.y - rad,
                            body.radius * 2.0,
                            body.radius * 2.0,
                        ],
                        c.transform,
                        gl,
                    );
                    for line_raw in body.force_lines {
                        line(body.inverse_color, 1.0, line_raw, c.transform, gl)
                    }
                }
            });
            window.set_title(format!(
                "Orbital | {:03} fps | Drawing {:03} Bodies | Max Speed is currently {:.01} | Largest Body Is {:03}",
                fps_counter.tick(),
                bodies.len(),
                bodies
                    .iter()
                    .map(|b| b.particle.vel.mag())
                    .reduce(f64::max)
                    .unwrap(),
                bodies
                    .iter()
                    .map(|b| b.radius)
                    .reduce(f64::max)
                    .unwrap()
            ));
            for body in &mut bodies {
                body.force_lines = vec![];
            }

            if bodies.len() < max_bodies as usize {
                for _ in 0..(max_bodies as usize - bodies.len()) {
                    bodies.push(Body::random());
                }
            }
        } else if let Some([x, y]) = e.mouse_cursor_args() {
            mouseX = x;
            mouseY = y;
        } else {
        }
    }
}
