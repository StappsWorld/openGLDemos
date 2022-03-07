extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate tokio;

use ::image::Rgba;
use glutin_window::GlutinWindow as Window;
use noise::{NoiseFn, Seedable};
use noise_visualize::{FPSCounter, *};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{
    AdvancedWindow, AfterRenderEvent, EventLoop, MouseCursorEvent, RenderArgs, UpdateEvent,
};
use rand::Rng;
use rayon::prelude::*;
use yalal::{line::Line, matrix::Matrix, vector::*};

#[tokio::main]
async fn main() {
    let opengl = OpenGL::V2_1;

    let mut window: Window = WindowSettings::new("Ulam Spiral", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut rng = rand::thread_rng();

    let mut events = Events::new(EventSettings::new());

    #[allow(non_upper_case_globals)]
    const rez: usize = 3;
    #[allow(non_upper_case_globals)]
    const half_rez: usize = rez / 2;
    #[allow(non_upper_case_globals)]
    const rez_f32: f32 = rez as f32;
    #[allow(non_upper_case_globals)]
    const rez_f64: f64 = rez as f64;
    #[allow(non_upper_case_globals)]
    const half_rez_f32: f32 = rez_f32 / 2.0;
    #[allow(non_upper_case_globals)]
    const half_rez_f64: f64 = rez_f64 / 2.0;
    #[allow(non_upper_case_globals)]
    const cols: usize = 1 + WINDOW_WIDTH as usize / rez;
    #[allow(non_upper_case_globals)]
    const rows: usize = 1 + WINDOW_HEIGHT as usize / rez;

    let mut noise = noise::SuperSimplex::new();
    noise.set_seed(rng.gen());

    let mut time = 0.0;

    events.set_max_fps(10000);
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
                let field = (0..cols)
                    .into_par_iter()
                    .map(|i| {
                        let i_f32 = i as f32;
                        (0..rows)
                            .into_par_iter()
                            .map(|j| {
                                let j_f32 = j as f32;
                                let x: f64 = (i_f32 * rez_f32) as f64;
                                let y: f64 = (j_f32 * rez_f32) as f64;
                                (x, y, noise.get([x, y, time]) as f32)
                            })
                            .collect::<Vec<(f64, f64, f32)>>()
                    })
                    .collect::<Vec<Vec<(f64, f64, f32)>>>();
                for col in field {
                    for (x, y, noise) in col {
                        // println!("{}", noise);
                        let noise = if noise < 0.0 { noise * -1.0 } else { noise } * 0.5;
                        rectangle(
                            [1.0, 1.0, 1.0, noise],
                            [x - half_rez_f64, y - half_rez_f64, rez_f64, rez_f64],
                            c.transform,
                            gl,
                        );
                    }
                }
            });
            window.set_title(format!("Noise Visualizer | {:03} fps", fps_counter.tick()));
        } else if let piston::Event::Input(i, _) = e {
            match i {
                piston::Input::Button(b) => match (b.state, b.button) {
                    (piston::ButtonState::Release, piston::Button::Keyboard(k)) => match k {
                        piston::Key::Space => {
                            noise.set_seed(rng.gen::<u32>());
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        } else if let Some(u) = e.update_args() {
            time += u.dt;
        }
    }
}
