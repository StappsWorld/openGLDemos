use crate::{vector::Vector, *};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Particle {
    pub pos: Vector,
    pub vel: Vector,
    pub acc: Vector,
    pub max_speed: f64,
    pub color: [f32; 4],
}
impl Particle {
    pub fn new<T: 'static + Into<f64> + Copy>(x: T, y: T, max_speed: T, color : Option<[f32; 4]>) -> Particle {
        Particle {
            pos: Vector {
                x: x.into(),
                y: y.into(),
            },
            vel: Vector::default(),
            acc: Vector::default(),
            max_speed: max_speed.into(),
            color: color.unwrap_or(WHITE),
        }
    }

    pub fn random() -> Particle {
        let mut rng = rand::thread_rng();
        Particle {
            pos: Vector {
                x: rng.gen_range(0.0..WINDOW_WIDTH as f64),
                y: rng.gen_range(0.0..WINDOW_HEIGHT as f64),
            },
            vel: Vector::random2D(),
            acc: Vector::random2D(),
            max_speed: rng.gen_range(0.0..4.0 as f64),
            color: random_color(),
        }
    }

    pub fn update(&mut self) {
        self.vel += self.acc;
        self.vel.limit_mag(self.max_speed);
        self.pos += self.vel;
        self.acc *= 0.0;

        self.acc += self.vel * 0.07 * -1.0;
    }

    pub fn apply_force(&mut self, force: Vector) {
        self.acc += force;
    }

    pub fn show(&self, gl: &mut GlGraphics, args: &RenderArgs, particle_size: f64) {
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
