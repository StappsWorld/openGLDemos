extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use ::image::Rgba;
use gl::types::GLuint;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings, GlyphCache, Filter};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop, MouseCursorEvent, RenderArgs};
use rand::Rng;
use hilbert_curve::{FPSCounter, vector::Vector};

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

const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;

const order : u32 = 3;

pub struct hilbert {
    points : Vec<Vector>
} impl hilbert {
    pub fn new() -> hilbert {
        let mut points = Vec::new();
        points.push(Vector::new(0.0, 0.0));
        points.push(Vector::new(0.0, 1.0));
        points.push(Vector::new(1.0, 1.0));
        points.push(Vector::new(1.0, 0.0));

        hilbert { points }
    }

    pub fn get(&self, i: usize) -> Vector {
        let mut v = self.points[i & 3];
        match (i >> 2) & 3 {
            0 => {
                let temp = v.x;
                v.x = v.y;
                v.y = temp;
            },
            1 => v.y += order as f64,
            2 => {
                v.x += order as f64;
                v.y += order as f64;
            },
            3 => {
                let temp = 1.0 - v.x;
                v.x = 1.0 - v.y;
                v.y = temp;
                v.x += order as f64;
            },
            _ => unreachable!(),
        }

        v
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Hilbert Curve", [WINDOW_WIDTH, WINDOW_HEIGHT])
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

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new(font, (), texture_settings)
        .expect("Could not load font");

    let N = 2u32.pow(order);
    let total = N * N;
    let mut path : Vec<Vector> = vec![Vector::default(); total as usize];

    let translate = Vector::new(WINDOW_WIDTH as f64 / 2.0, WINDOW_HEIGHT as f64 / 2.0);

    let h = hilbert::new();

    for i in 0..total as usize {
        path[i] = h.get(i);
        let len = WINDOW_WIDTH / (N * order);
        path[i] *= len as f64;
        path[i] -= Vector::new(len as f64 / 2.0, len as f64 / 2.0);
    }

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
                
                let mut old = path[0] + translate;
                let mut current = path[1] + translate;
                let transform = c.transform;
                for i in 2..total as usize + 1 {
                    line(WHITE, 1.0, [old.x, old.y, current.x, current.y], transform, gl);
                    ellipse(WHITE, [old.x - 2.5, old.y - 2.5, 5.0, 5.0], transform, gl);
                    text(WHITE, 7, (i - 2).to_string().as_str(), glyphs, transform.trans_pos([old.x - 10.0, old.y - 10.0]).flip_v(), gl).unwrap();
                    old = current;
                    current = *path.get(i).unwrap_or(&Vector::default()) + translate;
                }
                text(WHITE, 7, (total - 1).to_string().as_str(), glyphs, transform.trans_pos([old.x - 10.0, old.y - 10.0]).flip_v(), gl).unwrap();
                ellipse(WHITE, [old.x - 2.5, old.y - 2.5, 5.0, 5.0], transform, gl);


            });

            // Draw framebuffer to screen
            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
                Image::new().draw(&texture, &c.draw_state, c.transform, gl);
            });
            window.set_title(format!("Hilbert Curve | {:03} fps | Order {}", fps_counter.tick(), order));
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
