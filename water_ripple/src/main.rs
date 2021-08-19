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
use piston::{
    AdvancedWindow, EventLoop, MouseCursorEvent, MouseRelativeEvent, PressEvent, RenderArgs,
};
use rand::Rng;
use water_ripple::{vector::Vector, FPSCounter, *};

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Water Ripple", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let canvases_orig = [
        image::ImageBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        image::ImageBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT),
    ];
    let mut canvases = canvases_orig.clone();

    let mut textures: [Texture; 2] = [
        Texture::from_image(&canvases[0], &TextureSettings::new()),
        Texture::from_image(&canvases[1], &TextureSettings::new()),
    ];

    let mut fbo;
    let mut fbos: [GLuint; 2] = [0, 0];
    unsafe {
        gl::GenFramebuffers(2, fbos.as_mut_ptr());
        for (i, texture) in textures.iter().enumerate() {
            fbo = fbos[i];
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture.get_id(),
                0,
            );
        }
    }

    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut rng = rand::thread_rng();

    const cols: u32 = WINDOW_WIDTH;
    const rows: u32 = WINDOW_HEIGHT;

    const dampening: f64 = 0.7;

    let mut switch: bool = false;

    let (mut mouseX, mut mouseY): (u32, u32) = (0, 0);

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(10000);
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            fbo = fbos[switch as usize];

            let current_texture = textures.get_mut(switch as usize).unwrap();

            let previous_canvas = canvases.get(!switch as usize).unwrap().clone();
            let current_canvas = canvases.get_mut(switch as usize).unwrap();

            // Draw to the framebuffer
            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            }
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);

                for i in 1..cols - 1 {
                    for j in 1..rows - 1 {
                        let pixels = [
                            previous_canvas.get_pixel(i - 1, j),
                            previous_canvas.get_pixel(i + 1, j),
                            previous_canvas.get_pixel(i, j - 1),
                            previous_canvas.get_pixel(i, j + 1),
                        ];

                        let pixel = val_to_pixel(
                            ((pixels.iter().map(|p| rgba_to_u8(**p) as u16).sum::<u16>() / 2
                                - rgba_to_u8(*current_canvas.get_pixel(i, j)) as u16)
                                as f64
                                * dampening) as u16,
                        );
                        current_canvas.put_pixel(i, j, Rgba::<u8>::from(pixel))
                    }
                }

                current_texture.update(current_canvas);
            });

            // Draw framebuffer to screen
            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            }
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
                Image::new().draw(current_texture, &c.draw_state, c.transform, gl);
            });
            window.set_title(format!("Water Ripple | {:03} fps", fps_counter.tick()));

            switch = !switch;
        } else if let Some([x_raw, y_raw]) = e.mouse_cursor_args() {
            mouseX = x_raw as u32;
            mouseY = y_raw as u32;
        } else if let piston::Event::Input(i, _) = e {
            // Input stuff here
            match i {
                piston::Input::Button(b) => match (b.state, b.button) {
                    (piston::ButtonState::Release, piston::Button::Keyboard(k)) => match k {
                        piston::Key::Space => {
                            canvases = canvases_orig.clone();
                        }
                        _ => (),
                    },
                    (
                        piston::ButtonState::Press,
                        piston::Button::Mouse(piston::MouseButton::Left),
                    ) => {
                        let canvas = canvases.get_mut(!switch as usize).unwrap();
                        canvas.put_pixel(mouseX, mouseY, Rgba::<u8>::from(val_to_pixel(255)));
                    }
                    _ => (),
                },
                _ => (),
            }
        } else {
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

fn val_to_pixel(val: u16) -> [u8; 4] {
    let safe_val = u16_to_u8(val);
    [safe_val, safe_val, safe_val, 255]
}

fn rgba_to_u8(rgba: Rgba<u8>) -> u8 {
    let [r, g, b, _] = rgba.0;
    let val = r as u16 + g as u16 + b as u16 / 3;
    u16_to_u8(val)
}
