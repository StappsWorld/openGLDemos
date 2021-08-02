extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use colors_transform::Color;
use gl::types::GLuint;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop, MouseCursorEvent};
use rand::Rng;
use sunflower::FPSCounter;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

#[derive(Debug, Clone, Copy)]
struct Particle {
    pub position: [f64; 2],
    pub rgb: [f32; 4],
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window =
        WindowSettings::new("Phyllotactic Pattern", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let texture_buf = vec![0u8; WINDOW_WIDTH as usize * WINDOW_HEIGHT as usize];
    let texture = Texture::from_memory_alpha(
        &texture_buf,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        &TextureSettings::new(),
    )
    .expect("texture");

    let fbo;
    unsafe {
        let mut fbos: [GLuint; 1] = [0];
        gl::GenFramebuffers(1, fbos.as_mut_ptr());
        fbo = fbos[0];
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            texture.get_id(),
            0,
        );
    }
    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut n: f64 = 0.0;
    let c: f64 = 8.0;

    let mut current_angle: f64 = rand::thread_rng().gen_range(0.0..360.0);

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(10000);

    let mut particles: Vec<Particle> = vec![];
    let mut switching = false;

    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {

            if particles.len() == 0 {
                continue;
            }
            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            }
            gl.draw(args.viewport(), |context, gl| {
                for particle in particles.clone() {
                    ellipse(
                        particle.rgb,
                        [particle.position[0], particle.position[1], 2.0, 2.0],
                        context.transform,
                        gl,
                    );
                }

                particles = vec![];

                if switching {
                    clear(BLACK, gl);
                    switching = false;
                }
            });

            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }
            gl.draw(args.viewport(), |context, gl| {
                clear(BLACK, gl);
                Image::new().draw(&texture, &context.draw_state, context.transform, gl);
            });

            window.set_title(format!(
                "Phyllotactic Pattern | {:03} fps | Dots Drawn {:05.0} | Current Angle Being Drawn {}",
                fps_counter.tick(), n, current_angle
            ));
        } else if let Some(_) = e.mouse_cursor_args() {
        } else {
            for _ in 0..100 {
                if switching {
                    n = 0.0;
                    current_angle = rand::thread_rng().gen_range(0.0..360.0);
                    break;
                }

                let a = theta(n, current_angle);
                let r = r(c, n);
                let x = get_x(a, r) + (WINDOW_WIDTH as f64 / 2.0);
                let y = get_y(a, r) + (WINDOW_HEIGHT as f64 / 2.0);

                let rgb =
                    colors_transform::Hsl::from((((a - r) / 10.0) % 360.0) as f32, 90.0, 50.0)
                        .to_rgb();

                particles.push(Particle {
                    position: [x, y],
                    rgb: from_rgba([rgb.get_red(), rgb.get_green(), rgb.get_blue(), 1.0]),
                });

                n += 0.1;

                switching = n > 20000.0;
            }
        }
    }
}

fn from_rgba(pack: [f32; 4]) -> [f32; 4] {
    let [r, g, b, a] = pack;
    let [r_f, g_f, b_f] = [r / 255.0, g / 255.0, b / 255.0];
    [r_f, g_f, b_f, a]
}

fn theta(n: f64, angle: f64) -> f64 {
    n * angle
}

fn r(c: f64, n: f64) -> f64 {
    c * n.sqrt()
}

fn get_x(theta: f64, r: f64) -> f64 {
    r * theta.cos()
}

fn get_y(theta: f64, r: f64) -> f64 {
    r * theta.sin()
}
