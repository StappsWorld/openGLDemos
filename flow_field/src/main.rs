extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use ::image::Rgba;
use flow_field::{vector::Vector, FPSCounter, Particle, WINDOW_HEIGHT, WINDOW_WIDTH};
use gl::types::GLuint;
use glutin_window::GlutinWindow as Window;
use noise::{NoiseFn, Perlin, Seedable};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop, MouseCursorEvent, RenderArgs};
use rand::Rng;
use std::time::{Duration, Instant};

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

const PI: f64 = 3.14159265358979323;
const TWO_PI: f64 = 6.28318530717958647;
const PI_OVER_2: f64 = PI / 2.0;
const PI_OVER_4: f64 = PI / 4.0;
const PI_OVER_6: f64 = PI / 6.0;
const PI_OVER_8: f64 = PI / 8.0;
const PI_OVER_16: f64 = PI / 16.0;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window =
        WindowSettings::new("Perlin Noise Flow Field", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut rng = rand::thread_rng();
    let mut perlin = Perlin::default().set_seed(rng.gen_range(0..u32::MAX));

    const inc: f64 = 0.01;
    const scl: f64 = 20.0;

    let cols = (WINDOW_WIDTH as f64 / scl).floor() as u32;
    let rows = (WINDOW_HEIGHT as f64 / scl).floor() as u32;

    let flow_field_size = rows * cols;
    const max_particles: u32 = 1500;
    const particle_size: f64 = 4.0;

    let mut now = Instant::now();

    let mut flow_field: Vec<Vector> = vec![Vector::default(); flow_field_size as usize];

    let mut particles: Vec<Particle> = vec![];

    for _ in 0..max_particles {
        particles.push(Particle::random());
    }

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(200);
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);

                for y in 0..rows {
                    for x in 0..cols {
                        let index: usize = (x + y * cols) as usize;
                        let v = flow_field[index];

                        let transform = c
                            .transform
                            .trans(x as f64 * scl, y as f64 * scl)
                            .rot_deg(v.heading());
                        line(
                            [1.0, 1.0, 1.0, 0.5],
                            scl * 0.025,
                            [0., 0., scl as f64, 0.],
                            transform,
                            gl,
                        );
                    }
                }

                for particle in particles.clone() {
                    if particle.on_screen() {
                        ellipse(
                            particle.color,
                            [particle.pos.x, particle.pos.y, particle_size, particle_size],
                            c.transform,
                            gl,
                        );
                    }
                }
            });
            window.set_title(format!(
                "Perlin Noise Flow Field | {:03} fps | Lines Drawn {:03} | Particles Drawn {:03}",
                fps_counter.tick(),
                flow_field_size,
                max_particles
            ));
        } else if let piston::Event::Input(i, _) = e {
            // Input stuff here
            match i {
                piston::Input::Button(b) => match (b.state, b.button) {
                    (piston::ButtonState::Release, piston::Button::Keyboard(k)) => match k {
                        piston::Key::Space => {
                            perlin = Perlin::default().set_seed(rng.gen_range(0..u32::MAX));
                            now = Instant::now();
                            particles = vec![];
                            for _ in 0..max_particles {
                                particles.push(Particle::random());
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        } else {
            let mut yoff: f64 = 0.0;
            for y in 0..rows {
                let mut xoff: f64 = 0.0;
                for x in 0..cols {
                    let index: usize = (x + y * cols) as usize;
                    let angle =
                        perlin.get([xoff, yoff, now.elapsed().as_secs_f64() / 10.0]) * TWO_PI;
                    //let angle = perlin.get([xoff, yoff]) * TWO_PI;
                    let mut v = Vector::from_angle(angle);
                    v.set_mag(0.1);
                    flow_field[index] = v;

                    xoff += inc;
                }
                yoff += inc;
            }

            for particle in &mut particles {
                particle.update();
                particle.edges();
                particle.follow(&flow_field, &scl, &cols, &rows);
            }
        }
    }
}

