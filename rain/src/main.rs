extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use colors_transform::Color;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop};
use rain::FPSCounter;
use rand::Rng;
use std::time::{Duration, Instant};

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

const window_width: u32 = 2560;
const window_height: u32 = 1440;

#[derive(Default)]
pub struct Drop {
    pub x: f64,
    pub y: f64,
    pub yspeed: f64,
    pub xspeed: f64,
    pub z: f64,
    pub len: f64,
    pub thick: f64,
}
impl Drop {
    pub fn reset(&mut self) {
        self.x = rand::thread_rng().gen_range(-250.0..window_width as f64);
        self.y = rand::thread_rng().gen_range(-1000.0..-100.0);
        self.yspeed = map_range((0.0, 20.0), (4.0, 10.0), self.z);
        self.xspeed = map_range((0.0, 20.0), (1.0, 3.0), self.z);
        self.z = rand::thread_rng().gen_range(0.0..20.0);
        self.len = map_range((0.0, 20.0), (10.0, 20.0), self.z);
        self.thick = map_range((0.0, 20.0), (0.1, 1.0), self.z);
    }

    pub fn fall(&mut self) {
        self.y += self.yspeed;
        self.x += self.xspeed;
        self.yspeed += map_range((0.0, 20.0), (0.0, 0.2), self.z);
        self.xspeed += map_range((0.0, 20.0), (0.0, 0.02), self.z);

        if self.y > window_height as f64 {
            self.reset();
        }
    }

    pub fn show(&self, transform: graphics::math::Matrix2d, gl: &mut GlGraphics) {
        use graphics::*;

        line(
            from_rgba([55.0, 69.0, 74.0, 1.0]),
            self.thick,
            [self.x, self.y, self.x + 5.0, self.y + self.len],
            transform,
            gl,
        );
    }
}

#[derive(Debug, Clone)]
pub struct Lightning {
    pub x: f64,
    pub max_r: f64,
    pub time_started: Instant,
    pub total_time: Duration,
    pub active: bool,
}
impl Lightning {
    pub fn show(&mut self, transform: graphics::math::Matrix2d, gl: &mut GlGraphics) {
        use graphics::*;

        let percent_done =
            self.time_started.elapsed().as_secs_f64() / self.total_time.as_secs_f64();
        if percent_done > 1.0 {
            self.active = false;
        } else {
            let mut i = 0.0;
            while i < 1.0 {
                let circle = if percent_done < 0.6 {
                    // Wind up
                    i * percent_done
                } else if percent_done >= 0.6 && percent_done <= 0.8 {
                    // Big flash
                    i * percent_done + map_range((0.0, 1.0), (0.0, 0.1), i)
                } else {
                    // Wind down
                    i * (1.0 - map_range((0.8, 1.0), (0.2, 0.99), percent_done))
                };

                let rgb = colors_transform::Hsl::from(187.0, 41.1, 80.0 * circle as f32).to_rgb();

                ellipse(
                    from_rgba([
                        rgb.get_red(),
                        rgb.get_green(),
                        rgb.get_blue(),
                        circle as f32,
                    ]),
                    ellipse::circle(self.x, 0.0, self.max_r * (1.0 - i) as f64),
                    transform,
                    gl,
                );

                i += 0.05;
            }
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Rain", [window_width, window_height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut gl = GlGraphics::new(opengl);

    let mut start = Instant::now();

    let mut drops = vec![];
    const total_drops: usize = 60000;

    for _ in 0..total_drops {
        drops.push({
            let mut d = Drop::default();
            d.reset();
            d
        });
    }

    let mut lightning: Lightning = Lightning {
        x: 0.0,
        max_r: 0.0,
        time_started: Instant::now(),
        total_time: Duration::from_secs(1),
        active: false,
    };

    let mut next_lightning = Duration::from_secs(rand::thread_rng().gen_range(2..4));

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let background : [f32; 4] = from_rgba([17.0, 29.0, 38.0, 1.0]);

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(1000);

    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            let current_fps = fps_counter.tick();

            let mut drawn_drops: usize = 0;

            if start.elapsed() > next_lightning && !lightning.active {
                lightning = Lightning {
                    x: rand::thread_rng().gen_range(0.0..window_width as f64),
                    max_r: rand::thread_rng().gen_range(200.0..400.0),
                    time_started: Instant::now(),
                    total_time: Duration::from_secs(rand::thread_rng().gen_range(2..10)),
                    active: true,
                };

                next_lightning = Duration::from_secs(rand::thread_rng().gen_range(12..30));
                start = Instant::now();
            }

            gl.draw(args.viewport(), |c, gl| {
                clear(background, gl);

                if lightning.active {
                    lightning.show(c.transform, gl);
                }

                for drop_index in 0..total_drops {
                    let drop = drops.get_mut(drop_index).unwrap();
                    drop.fall();
                    if drop.y + drop.len >= 0.0
                        && drop.x + drop.xspeed >= 0.0
                        && drop.x <= window_width as f64
                    {
                        drop.show(c.transform, gl);
                        drawn_drops += 1;
                    }
                }
            });
            window.set_title(format!(
                "Rain | {} fps | Drops Drawn : {} | {}",
                current_fps,
                drawn_drops,
                if lightning.active {
                    "Drawing Lightning"
                } else {
                    "Not Drawing Lightning"
                },
            ));
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
