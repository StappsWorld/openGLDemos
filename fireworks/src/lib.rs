pub mod vector;

use lazy_static::lazy_static;
use opengl_graphics::{GlGraphics, Texture};
use piston::RenderArgs;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use vector::Vector;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const WINDOW_WIDTH: u32 = 1920;
pub const WINDOW_HEIGHT: u32 = 1080;

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
    pub color: [f32; 4],
    pub lifespan: i16,
    pub firework: bool,
}
impl Particle {
    pub fn new(x: f64, y: f64, firework: bool, color: [f32; 4]) -> Particle {
        let mut rng = rand::thread_rng();

        Particle {
            pos: Vector { x: x, y: y },
            vel: if firework {
                Vector {
                    x: 0.0,
                    y: rand::Rng::gen_range(&mut rng, -((WINDOW_HEIGHT / 60) as f64)..-((WINDOW_HEIGHT / 75) as f64)),
                }
            } else {
                Vector::random2D()
            },
            acc: Vector { x: 0.0, y: 0.0 },
            color: color,
            lifespan: rand::Rng::gen_range(&mut rng, 200..255),
            firework: firework,
        }
    }

    pub fn update(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
        self.acc *= 0.0;
        if !self.firework {
            self.lifespan -= 3;
            self.color[3] = map_range((0.0, 255.0), (0.0, 1.0), self.lifespan as f64) as f32;
        }
    }

    pub fn applyForce(&mut self, force: Vector) {
        self.acc += force;
    }

    pub fn show(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        if self.on_screen() {
            gl.draw(args.viewport(), |c, gl| {
                ellipse(
                    self.color,
                    [self.pos.x, self.pos.y, 4.0, 4.0],
                    c.transform,
                    gl,
                );
            });
        }
    }

    pub fn on_screen(&self) -> bool {
        self.pos.x > 0.0
            && self.pos.x <= WINDOW_WIDTH as f64
            && self.pos.y > 0.0
            && self.pos.y <= WINDOW_HEIGHT as f64
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Firework {
    pub firework: Particle,
    pub exploded: bool,
    pub particles: Vec<Particle>,
    pub highest_particle: Option<usize>,
}
impl Firework {
    pub fn new() -> Firework {
        let mut rng = rand::thread_rng();

        let firework = Particle::new(
            rand::Rng::gen_range(&mut rng, 0.0..WINDOW_WIDTH as f64),
            WINDOW_HEIGHT as f64,
            true,
            WHITE,
        );

        Firework {
            firework: firework,
            exploded: false,
            particles: vec![],
            highest_particle: None,
        }
    }

    pub fn update(&mut self, gravity: Vector) {
        if !self.exploded {
            self.firework.applyForce(gravity);
            self.firework.update();
            if self.firework.vel.y >= 1.0 {
                self.exploded = true;
                self.explode();
            }
        } else {
            for p in &mut self.particles {
                p.applyForce(gravity);
                p.update();
            }
        }
    }

    pub fn show(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        if !self.exploded {
            self.firework.show(gl, args);
        } else {
            for p in &self.particles {
                p.show(gl, args);
            }
        }
    }

    pub fn explode(&mut self) {
        for i in 0..100 {
            let p = Particle::new(
                self.firework.pos.x,
                self.firework.pos.y,
                false,
                random_color(),
            );
            self.particles.push(p);
            match self.highest_particle {
                Some(index) => if p.vel.y < self.particles.get(index).unwrap().vel.y {
                    self.highest_particle = Some(i);
                }
                None => self.highest_particle = Some(i),
            }
        }
    }
}

pub fn random_color() -> [f32; 4] {
    let mut rng = rand::thread_rng();
    [
        rand::Rng::gen_range(&mut rng, 0.0..1.0),
        rand::Rng::gen_range(&mut rng, 0.0..1.0),
        rand::Rng::gen_range(&mut rng, 0.0..1.0),
        1.0,
    ]
}

fn from_rgba(pack: [f32; 4]) -> [f32; 4] {
    let [r, g, b, a] = pack;
    let [r_f, g_f, b_f] = [r / 255.0, g / 255.0, b / 255.0];
    [r_f, g_f, b_f, a]
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
