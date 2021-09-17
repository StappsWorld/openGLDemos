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
use vector_rotation::{particle::Particle, FPSCounter, *};
use yalal::{line::Line, matrix::Matrix, vector::*};

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window =
        WindowSettings::new("YALAL Vector Rotation Test", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut rng = rand::thread_rng();

    let still_vector = Vector::from_angle(-45) * 200.0;
    let (sx, sy) = still_vector.x_y();

    let mut vector1 = Vector::standard_unit() * 300.0;
    let mut vec1_speed = 50.0;
    let mut vector2 = Vector::standard_unit() * 300.0;
    let mut vec2_speed = 25.0;
    let mut flip = false;
    let mut colliding : Option<Vector> = None;

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(200);
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
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
                line(
                    LIGHT_GRAY,
                    1.0,
                    [0.0, -center_y, 0.0, center_y],
                    transform_axis,
                    gl,
                );

                let (v1x, v1y) = vector1.x_y();
                line(GRAY, 1.5, [0.0, 0.0, v1x, -v1y], transform_axis, gl);

                let (v2x, v2y) = vector2.x_y();
                line(GRAY, 1.5, [0.0, 0.0, v2x, -v2y], transform_axis, gl);

                let diff = vector2 - vector1;
                let (vax, vay) = diff.x_y();
                let from_v1_trans = transform_axis.trans(v1x, -v1y);
                line(
                    if colliding.is_some() { BLUE } else { RED },
                    1.5,
                    [0.0, 0.0, vax, -vay],
                    from_v1_trans,
                    gl,
                );

                let midpoint = diff * 0.5;
                let (max, may) = midpoint.x_y();
                let mid_trans = from_v1_trans.trans(max, -may);
                rectangle(
                    WHITE,
                    [-2.0, -2.0, 4.0, 4.0],
                    mid_trans,
                    gl,
                );

                line(
                    GREEN,
                    1.5,
                    [0.0, 0.0, -vay * 0.2, -vax * 0.2],
                    mid_trans,
                    gl
                );

                line(
                    MAGENTA,
                    1.5,
                    [0.0, 0.0, vay * 0.2, vax * 0.2],
                    mid_trans,
                    gl
                );

                line(
                    GRAY,
                        1.5,
                        [0.0, 0.0, sx, -sy],
                        transform_axis,
                        gl
                );

                if let Some(v) = colliding {
                    let (vx, vy) = v.x_y();
                    line(
                        YELLOW,
                        1.5,
                        [0.0, 0.0, vx, -vy],
                        transform_axis,
                        gl
                    );
                    ellipse(
                        YELLOW,
                        [vx - 5.0, -vy - 5.0, 10.0, 10.0],
                        transform_axis,
                        gl
                    );
                }

            });
            window.set_title(format!("YALAL Vector Rotation Test | {:03} fps | Vector1 heading is currently {:.02} | Vector2 heading is currently {:.02} | {}", fps_counter.tick(), vector1.heading(), vector2.heading(), if flip { "Flipped" } else { "Not flipped" }));
        } else if let Some(args) = piston::UpdateEvent::update_args(&e) {
            vector1.rotate(args.dt * -vec1_speed);
            if vector1.heading() < -360.0 {
                vector1.set_heading(vector1.heading() % 360.0)
            }
            vector2.rotate(args.dt * -vec2_speed);
            if vector2.heading() < -360.0 {
                vector2.set_heading(vector2.heading() % 360.0)
            }

            let diff = vector1.heading() - vector2.heading();
            if (diff < 1e-8 && diff > -1e-8) || (diff > 180.0 - 1e-8 && diff < 180.0 + 1e-8) {
                flip = !flip;
            }

            colliding = line_intersection(vector1, vector2, Vector::new(0, 0), still_vector);
        } else if let piston::Event::Input(i, _) = e {
            // Input stuff here
            match i {
                piston::Input::Button(b) => match (b.state, b.button) {
                    (piston::ButtonState::Release, piston::Button::Keyboard(k)) => match k {
                        piston::Key::Equals => {
                            vec1_speed -= 5.0;
                        },
                        piston::Key::Minus => {
                            vec1_speed += 5.0;
                        },
                        piston::Key::RightBracket => {
                            vec2_speed -= 5.0;
                        },
                        piston::Key::LeftBracket => {
                            vec2_speed += 5.0;
                        }
                        piston::Key::Space => {
                            vector1 = Vector::standard_unit() * 300.0;
                            vec1_speed = 50.0;
                            vector2 = Vector::standard_unit() * 300.0;
                            vec2_speed = 25.0;
                            flip = false;
                            colliding = None;
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
