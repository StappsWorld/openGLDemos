use ::image::Rgba;
use gl::types::GLuint;
use glutin_window::GlutinWindow as Window;
use mandelbrot::FPSCounter;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop, MouseCursorEvent, RenderArgs};
use rand::Rng;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;
const PI : f64 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Mandelbrot Set", [WINDOW_WIDTH, WINDOW_HEIGHT])
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

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(10000);

    let mut rng = rand::thread_rng();

    let mut drawn = false;
    let max_iterations = 150;
    let mut minval = -1.7;
    let mut maxval = 2.3;
    let mut precision = 0.1;

    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            if drawn {
                gl.draw(args.viewport(), |c, g| {
                    clear(BLACK, g);
                    Image::new().draw(&texture, &c.draw_state, c.transform, g);
                });
            } else {
                unsafe {
                    gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
                }
                gl.draw(args.viewport(), |_context, gl| {
                    //clear(BLACK, gl);
                    for x in 0..WINDOW_WIDTH {
                        for y in 0..WINDOW_HEIGHT {
                            let mut a =
                                map_range((0.0, WINDOW_WIDTH as f64), (minval, maxval), x as f64);
                            let mut b =
                                map_range((0.0, WINDOW_WIDTH as f64), (minval, maxval), y as f64);

                            let ca = a;
                            let cb = b;

                            let mut n = 0;

                            while n < max_iterations {
                                let aa = a * a - b * b;
                                let bb = 2.0 * a * b;

                                a = aa + ca;
                                b = bb + cb;

                                if a + b > 16.0 {
                                    break;
                                }
                                n += 1;
                            }

                            let bright = if n == max_iterations {
                                0
                            } else {
                                let first =
                                    map_range((0.0, max_iterations as f64), (0.0, 1.0), n as f64);
                                map_range((0.0, 1.0), (0.0, 255.0), first.sqrt()).floor() as u8
                            };
                            canvas.put_pixel(x, y, Rgba([bright, bright, bright, 255]));
                        }
                    }
                });

                texture.update(&canvas);

                drawn = true;

                unsafe {
                    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                }
            }

            window.set_title(format!(
                "Mandelbrot Set | {:03} fps | Minval {} | Maxval {} | Precision {}",
                fps_counter.tick(),
                minval,
                maxval,
                precision
            ));
        } else if let Some([x, y]) = e.mouse_cursor_args() {
            // Mouse stuff here
        } else if let piston::Event::Input(i, _) = e {
            // Input stuff here
            match i {
                piston::Input::Button(b) => match (b.state, b.button) {
                    (piston::ButtonState::Release, piston::Button::Keyboard(k)) => match k {
                        piston::Key::A => {
                            drawn = false;
                            minval -= precision
                        }
                        piston::Key::D => {
                            drawn = false;
                            minval += precision;
                            if minval >= maxval {
                                minval = maxval - precision;
                                precision /= 10.0;
                                minval += precision;
                            }
                        }
                        piston::Key::W => {
                            drawn = false;
                            maxval += precision
                            
                        }
                        piston::Key::S => {
                            drawn = false;
                            maxval -= precision;
                            if maxval <= minval {
                                maxval = minval + precision;
                                precision /= 10.0;
                                maxval += precision;
                            }
                        }
                        piston::Key::Q => {
                            precision /= 10.0;
                        }
                        piston::Key::E => {
                            precision *= 10.0;
                        },
                        piston::Key::Space => {
                            drawn = false;
                            precision = 0.1;
                            maxval = 2.3;
                            minval = -1.7;
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        } else {
            //Do outside rendering stuff here
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
