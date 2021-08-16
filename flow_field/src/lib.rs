use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use rand::Rng;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub const WINDOW_WIDTH: u32 = 1920;
pub const WINDOW_HEIGHT: u32 = 1080;

pub mod vector;

use vector::Vector;

/// Measures Frames Per Second (FPS).
#[derive(Debug)]
pub struct FPSCounter {
    /// The last registered frames.
    last_second_frames: VecDeque<Instant>,
}

impl Default for FPSCounter {
    fn default() -> Self {
        FPSCounter::new()
    }
}

impl FPSCounter {
    /// Creates a new FPSCounter.
    pub fn new() -> FPSCounter {
        FPSCounter {
            last_second_frames: VecDeque::with_capacity(128),
        }
    }

    /// Updates the FPSCounter and returns number of frames.
    pub fn tick(&mut self) -> usize {
        let now = Instant::now();
        let a_second_ago = now - Duration::from_secs(1);

        while self
            .last_second_frames
            .front()
            .map_or(false, |t| *t < a_second_ago)
        {
            self.last_second_frames.pop_front();
        }

        self.last_second_frames.push_back(now);
        self.last_second_frames.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Particle {
    pub pos: Vector,
    pub vel: Vector,
    pub acc: Vector,
    pub max_speed : f64,
    pub color: [f32; 4],
}
impl Particle {
    pub fn new<T: 'static + Into<f64> + Copy>(x: T, y: T, max_speed : T) -> Particle {
        let mut p = Particle {
            pos: Vector {
                x: x.into(),
                y: y.into(),
            },
            vel: Vector::default(),
            acc: Vector::default(),
            max_speed: max_speed.into(),
            color: [0.0, 0.2, 0.0, 1.0],
        };
        p.update_color();
        p
    }

    pub fn random() -> Particle {
        let mut rng = rand::thread_rng();
        let mut p = Particle {
            pos: Vector {
                x: rng.gen_range(0.0..WINDOW_WIDTH as f64),
                y: rng.gen_range(0.0..WINDOW_HEIGHT as f64),
            },
            vel: Vector::random2D(),
            acc: Vector::random2D(),
            max_speed: rng.gen_range(0.0..4.0 as f64),
            color: [0.0, 0.2, 0.0, 1.0],
        };
        p.update_color();
        p
    }

    pub fn update(&mut self) {
        self.vel += self.acc;
        self.vel.limit_mag(self.max_speed);
        self.pos += self.vel;
        self.acc *= 0.0;

        self.acc += self.vel * 0.07 * -1.0;

        self.update_color();
    }

    pub fn update_color(&mut self) {
        let speed_percent = self.vel.mag() / self.max_speed;
        if speed_percent <= 0.5 {
            self.color[0] = 1.0 - map_range((0.0, 0.5), (0.0, 1.0), speed_percent) as f32;
            self.color[2] = map_range((0.0, 0.5), (0.0, 1.0), speed_percent) as f32;
        } else {
            self.color[0] = map_range((0.5, 1.0), (0.0, 1.0), speed_percent) as f32;
            self.color[2] = 1.0 - map_range((0.5, 1.0), (0.0, 1.0), speed_percent) as f32;
        }
    }

    pub fn apply_force(&mut self, force: Vector) {
        self.acc += force;
    }

    pub fn follow(&mut self, vectors : &Vec<Vector>, scl : &f64, cols : &u32, rows : &u32) {
        loop {
            let x = (self.pos.x / scl).floor() as usize;
            let y = (self.pos.y / scl).floor() as usize;
            let index = x + y * *cols as usize;
    
            if index >= (rows * cols) as usize {
                //println!("Would have panicked! Index is {}. This particle's position is {:?}, and its (x, y) is {:?}. Scl is {}. This particle is {}", index, self.pos, (x, y), scl, if self.on_screen() { "on screen" } else { "off screen" });
                self.edges();
            } else {
                let force = vectors[index];
                self.apply_force(force);
                break;
            }
        }

    }

    pub fn show(&self, gl: &mut GlGraphics, args: &RenderArgs, particle_size : f64) {
        use graphics::*;

        if self.on_screen() {
            gl.draw(args.viewport(), |c, gl| {
                ellipse(
                    self.color,
                    [self.pos.x, self.pos.y, particle_size, particle_size],
                    c.transform,
                    gl,
                );
            });
        }
    }

    pub fn edges(&mut self) {
        if !self.on_screen() {
            if self.pos.x >= WINDOW_WIDTH as f64 {
                self.pos.x = 0.0;
            } else if self.pos.x <= 0.0 {
                self.pos.x = WINDOW_WIDTH as f64 - 1.0;
            }
            if self.pos.y >= WINDOW_HEIGHT as f64 {
                self.pos.y = 0.0;
            } else if self.pos.y <= 0.0 {
                self.pos.y = WINDOW_HEIGHT as f64 - 1.0;
            }
        }
    }

    pub fn on_screen(&self) -> bool {
        self.pos.x > 0.0
            && self.pos.x < WINDOW_WIDTH as f64
            && self.pos.y > 0.0
            && self.pos.y < WINDOW_HEIGHT as f64
    }
}

pub fn from_rgba<T: 'static + Into<f64> + Copy>(pack: [T; 4]) -> [f32; 4] {
    let r_raw: f64 = pack[0].into();
    let g_raw: f64 = pack[1].into();
    let b_raw: f64 = pack[2].into();
    let a_raw: f64 = pack[3].into();

    let r = r_raw as f32;
    let g = g_raw as f32;
    let b = b_raw as f32;
    let a = a_raw as f32;

    let [r_f, g_f, b_f] = [r / 255.0, g / 255.0, b / 255.0];
    [r_f, g_f, b_f, a]
}

pub fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub fn random_color() -> [f32; 4] {
    let r = rand::thread_rng().gen_range(0.0..1.0);
    let g = rand::thread_rng().gen_range(0.0..1.0);
    let b = rand::thread_rng().gen_range(0.0..1.0);
    [r, g, b, 1.0]
}