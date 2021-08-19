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
use Example_Package::{FPSCounter, vector::Vector, particle::Particle, *};



fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Example Title", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut canvas = image::ImageBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut texture: Texture = Texture::from_image(&canvas, &TextureSettings::new());

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

    let mut rng = rand::thread_rng();


    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(10000);
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            // Draw to the framebuffer
            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            }
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
            });

            // Draw framebuffer to screen
            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
                Image::new().draw(&texture, &c.draw_state, c.transform, gl);
            });
            window.set_title(format!("Example Title | {:03} fps", fps_counter.tick()));
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
