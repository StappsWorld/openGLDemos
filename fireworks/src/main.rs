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
use Fireworks::Firework;
use Fireworks::{vector::Vector, FPSCounter, Particle, BLACK, WHITE, WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Fireworks", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut rng = rand::thread_rng();

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(144);

    let mut fireworks = vec![Firework::new()];

    let gravity = Vector {
        x: 0.0,
        y: WINDOW_HEIGHT as f64 / 6000.0,
    };

    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |_c, gl| {
                clear(BLACK, gl);

                for firework in &fireworks {
                    firework.show(gl, &args);
                }
            });

            let mut total_particles = 0;

            for firework in fireworks.clone() {
                if !firework.exploded {
                    total_particles += 1;
                } else {
                    for particle in firework.particles {
                        total_particles += if particle.on_screen() { 1 } else { 0 };
                    }
                };
            }

            window.set_title(format!(
                "Fireworks | {:03} fps | Active Fireworks {:02} | Total Particles On Screen {:04}",
                fps_counter.tick(),
                fireworks.len(),
                total_particles
            ));
        } else if let Some([x, y]) = e.mouse_cursor_args() {}
        else {
            if rng.gen_range(0.0..1.0) < 0.08 && fireworks.len() < 12 {
                fireworks.push(Firework::new());
            }

            for firework in &mut fireworks {
                firework.update(gravity);
            }

            fireworks = fireworks
                .iter()
                .filter(|firework| {
                    !(firework.exploded
                        && firework.particles.get(firework
                            .highest_particle.unwrap()).unwrap().pos.y > WINDOW_HEIGHT as f64 + 5.0)
                })
                .cloned()
                .collect();
        }
    }
}
