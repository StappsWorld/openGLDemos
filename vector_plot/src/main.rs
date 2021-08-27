extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use gl::types::GLuint;
use glutin_window::GlutinWindow as Window;
use image::{ImageBuffer, Rgba};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings, GlyphCache, Filter};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop, MouseCursorEvent, RenderArgs, ResizeEvent};
use rand::Rng;
use vector_plot::{particle::Particle, vector::Vector, FPSCounter, *};

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Vector Plotting", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut canvas = image::ImageBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut texture: Texture = Texture::from_image(&canvas, &TextureSettings::new());

    let mut fbo;
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

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new(font, (), texture_settings)
        .expect("Could not load font");

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut rng = rand::thread_rng();

    let mut first: Vector = Vector::random2D() * rng.gen_range(100.0..500.0);
    let mut second: Vector = Vector::random2D() * rng.gen_range(100.0..500.0);
    let mut added: Vector = first + second;

    let mut drawn = false;

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(240);
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            if !drawn {
                // Draw to the framebuffer
                unsafe {
                    gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
                }
                gl.draw(args.viewport(), |c, gl| {
                    clear(BLACK, gl);

                    let (window_width, window_height) = (args.window_size[0], args.window_size[1]);
                    let (center_x, center_y) = (window_width / 2.0, window_height / 2.0);

                    let transform_axis = c.transform.trans(center_x, center_y);

                    line(
                        LIGHT_GRAY,
                        1.0,
                        [-center_x, 0.0, center_x, 0.0],
                        transform_axis,
                        gl,
                    );

                    text(LIGHT_GRAY, 10, "X", glyphs, c.transform.trans(5.0, center_y - 15.0).flip_v(), gl).unwrap();

                    line(
                        LIGHT_GRAY,
                        1.0,
                        [0.0, -center_y, 0.0, center_y],
                        transform_axis,
                        gl,
                    );
                    text(LIGHT_GRAY, 10, "Y", glyphs, c.transform.trans(center_x + 5.0, window_height - 10.0).flip_v(), gl).unwrap();


                    let (first_x, first_y) = first.x_y();
                    let first_color = random_color();
                    let (second_x, second_y) = second.x_y();
                    let second_color = random_color();
                    let (added_x, added_y) = added.x_y();
                    let added_color = random_color();

                    line(
                        first_color,
                        1.0,
                        [0.0, 0.0, first_x, first_y],
                        transform_axis,
                        gl,
                    );

                    text(first_color, 10, "First", glyphs, transform_axis.flip_v().trans(first_x / 2.0 + 20.0, -first_y / 2.0 - 20.0), gl).unwrap();

                    let second_transform = c.transform.trans(center_x + first_x, center_y + first_y);

                    line(
                        second_color,
                        1.0,
                        [0.0, 0.0, second_x, second_y],
                        second_transform,
                        gl,
                    );

                    text(second_color, 10, "Second", glyphs, second_transform.flip_v().trans(second_x / 2.0 + 20.0, -second_y / 2.0 - 20.0), gl).unwrap();

                    line(
                        added_color,
                        1.0,
                        [0.0, 0.0, added_x, added_y],
                        transform_axis,
                        gl,
                    );

                    text(added_color, 10, "Added", glyphs, transform_axis.flip_v().trans(added_x / 2.0 + 20.0, -added_y / 2.0 - 20.0), gl).unwrap();


                });

                // Draw framebuffer to screen
                unsafe {
                    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                }
                drawn = true;
            }
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
                Image::new().draw(&texture, &c.draw_state, c.transform, gl);
            });
            window.set_title(format!("Vector Plotting | {:03} fps", fps_counter.tick()));
        } else if let Some(args) = e.resize_args() {
            drawn = false;
            let (window_width, window_height) = (args.window_size[0], args.window_size[1]);
            canvas = ImageBuffer::new(window_width as u32, window_height as u32);
            texture = Texture::from_image(&canvas, &TextureSettings::new());
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
        } else if let piston::Event::Input(i, _) = e {
            // Input stuff here
            match i {
                piston::Input::Button(b) => match (b.state, b.button) {
                    (piston::ButtonState::Release, piston::Button::Keyboard(k)) => match k {
                        piston::Key::Space => {
                            drawn = false;
                            first = Vector::random2D() * rng.gen_range(100.0..500.0);
                            second = Vector::random2D() * rng.gen_range(100.0..500.0);
                            added = first + second;
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
