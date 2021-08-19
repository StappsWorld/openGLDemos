extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use ::image::{GenericImage, GenericImageView, Rgba};
use dithering::{vector::Vector, FPSCounter, *};
use gl::types::GLuint;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop, MouseCursorEvent, RenderArgs};
use rand::Rng;
use std::convert::TryFrom;
use std::path::Path;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Dithering", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut rng = rand::thread_rng();

    let postiton_space = WINDOW_WIDTH as f64 / 3.0;

    let original_kitten_texture =
        graphics::Image::new().rect(graphics::rectangle::square(0.0, 0.0, postiton_space));
    let black_and_white_kitten_texture = graphics::Image::new().rect(graphics::rectangle::square(
        postiton_space * 2.0,
        0.0,
        postiton_space,
    ));
    let dithered_kitten_texture = graphics::Image::new().rect(graphics::rectangle::square(
        postiton_space * 3.0,
        0.0,
        postiton_space,
    ));

    let original_kitten_image = image::open("assets/kitten.jpg").unwrap();

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(10000);
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
                let mut kitten = original_kitten_image.clone();

                for y in 0..kitten.height() - 1 {
                    for x in 1..kitten.width() - 1 {
                        let mut color = kitten.get_pixel(x, y);
                        let oldR: f64 = color[0] as f64;
                        let oldG: f64 = color[1] as f64;
                        let oldB: f64 = color[2] as f64;
                        let factor = 4.0;
                        let newR: u8 = ((factor * oldR / 255.0).round() * (255.0 / factor)) as u8;
                        let newG: u8 = ((factor * oldG / 255.0).round() * (255.0 / factor)) as u8;
                        let newB: u8 = ((factor * oldB / 255.0).round() * (255.0 / factor)) as u8;
                        kitten.put_pixel(x, y, Rgba::<u8>::from([newR, newG, newB, 255]));

                        let errR = oldR - newR as f64;
                        let errG = oldG - newG as f64;
                        let errB = oldB - newB as f64;

                        let mut color = kitten.get_pixel(x + 1, y);
                        let mut r = color[0] as u16;
                        let mut g = color[1] as u16;
                        let mut b = color[2] as u16;
                        r += (errR * 7.0 / 16.0).floor() as u16;
                        g += (errG * 7.0 / 16.0).floor() as u16;
                        b += (errB * 7.0 / 16.0).floor() as u16;
                        kitten.put_pixel(
                            x + 1,
                            y,
                            Rgba::<u8>::from(convert_vec_to_array(
                                [r, g, b, 255]
                                    .iter()
                                    .map(|u| u16_to_u8(*u))
                                    .collect::<Vec<u8>>(),
                            )),
                        );

                        color = kitten.get_pixel(x - 1, y + 1);
                        r = color[0] as u16;
                        g = color[1] as u16;
                        b = color[2] as u16;
                        r += (errR * 3.0 / 16.0).floor() as u16;
                        g += (errG * 3.0 / 16.0).floor() as u16;
                        b += (errB * 3.0 / 16.0).floor() as u16;
                        kitten.put_pixel(
                            x - 1,
                            y + 1,
                            Rgba::<u8>::from(convert_vec_to_array(
                                [r, g, b, 255]
                                    .iter()
                                    .map(|u| u16_to_u8(*u))
                                    .collect::<Vec<u8>>(),
                            )),
                        );

                        color = kitten.get_pixel(x, y + 1);
                        r = color[0] as u16;
                        g = color[1] as u16;
                        b = color[2] as u16;
                        r += (errR * 5.0 / 16.0).floor() as u16;
                        g += (errG * 5.0 / 16.0).floor() as u16;
                        b += (errB * 5.0 / 16.0).floor() as u16;
                        kitten.put_pixel(
                            x,
                            y + 1,
                            Rgba::<u8>::from(convert_vec_to_array(
                                [r, g, b, 255]
                                    .iter()
                                    .map(|u| u16_to_u8(*u))
                                    .collect::<Vec<u8>>(),
                            )),
                        );

                        color = kitten.get_pixel(x + 1, y + 1);
                        r = color[0] as u16;
                        g = color[1] as u16;
                        b = color[2] as u16;
                        r += (errR * 1.0 / 16.0).floor() as u16;
                        g += (errG * 1.0 / 16.0).floor() as u16;
                        b += (errB * 1.0 / 16.0).floor() as u16;
                        kitten.put_pixel(
                            x + 1,
                            y + 1,
                            Rgba::<u8>::from(convert_vec_to_array(
                                [r, g, b, 255]
                                    .iter()
                                    .map(|u| u16_to_u8(*u))
                                    .collect::<Vec<u8>>(),
                            )),
                        );
                    }
                }

                let mut baw_kitten = original_kitten_image.clone();

                for y in 0..baw_kitten.height() {
                    for x in 0..baw_kitten.width() {
                        let color = baw_kitten.get_pixel(x, y);
                        let oldR: f64 = color[0] as f64;
                        let oldG: f64 = color[1] as f64;
                        let oldB: f64 = color[2] as f64;
                        let factor = 4.0;
                        let newR: u8 = ((factor * oldR / 255.0).round() * (255.0 / factor)) as u8;
                        let newG: u8 = ((factor * oldG / 255.0).round() * (255.0 / factor)) as u8;
                        let newB: u8 = ((factor * oldB / 255.0).round() * (255.0 / factor)) as u8;
                        baw_kitten.put_pixel(x, y, Rgba::<u8>::from([newR, newG, newB, 255]));
                    }
                }

                let original_texture =
                    Texture::from_image(&original_kitten_image.to_rgba8(), &TextureSettings::new());
                original_kitten_texture.draw(
                    &original_texture,
                    &DrawState::default(),
                    c.transform,
                    gl,
                );

                let baw_texture =
                    Texture::from_image(&baw_kitten.to_rgba8(), &TextureSettings::new());
                black_and_white_kitten_texture.draw(
                    &baw_texture,
                    &DrawState::default(),
                    c.transform,
                    gl,
                );

                let dithered_texture =
                    Texture::from_image(&kitten.to_rgba8(), &TextureSettings::new());
                dithered_kitten_texture.draw(
                    &dithered_texture,
                    &DrawState::default(),
                    c.transform,
                    gl,
                );
            });

            window.set_title(format!("Dithering | {:03} fps", fps_counter.tick()));
        }
    }
}

fn u16_to_u8(x: u16) -> u8 {
    if x > 255 {
        255
    } else {
        x as u8
    }
}

use std::convert::TryInto;

fn convert_vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
