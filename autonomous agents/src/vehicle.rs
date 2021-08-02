use crate::{vector::Vector, WINDOW_WIDTH, WINDOW_HEIGHT};
use glutin_window::map_key;
use graphics::types::Triangle;
use opengl_graphics::{GlGraphics};
use piston::RenderArgs;
use rand::Rng;

pub struct Vehicle {
    pub pos: Vector,
    pub vel: Vector,
    pub acc: Vector,
    pub r: f64,
    pub color : [f32; 4],
    pub max_speed : f64,
    pub max_force : f64,
}
impl Vehicle {
    pub fn new(x: f64, y: f64, color : [f32; 4]) -> Vehicle {
        let mut rng = rand::thread_rng();

        let max_speed = rng.gen_range(0.05..0.2);

        Vehicle {
            pos: Vector { x, y },
            vel: Vector::random2D(),
            acc: Vector { x: 0.0, y: 0.0 },
            r: 16.0,
            color : color,
            max_speed: max_speed,
            max_force: 0.0001,
        }
    }

    pub fn arrive(&self, target: Vector) -> Vector {
        let mut force = target - self.pos;
        let r = 100.0;
        let d = force.mag();
        if d < r {
            let m = crate::map_range((0.0, r), (0.0, self.max_speed), d);
            force *= m;
        } else {
            force.set_mag(self.max_speed);
        }
        force -= self.vel;
        force.limit_mag(self.max_force);
        force
    }

    pub fn persue(&self, vehicle: &Vehicle) -> Vector {
        let mut target = vehicle.pos;
        target += vehicle.vel * 10.0;
        self.seek(target)
    }

    pub fn evade(&self, target: &Vehicle) -> Vector {
        -self.persue(&target)
    }

    pub fn seek(&self, target: Vector) -> Vector {
        let mut desired = target - self.pos;
        desired.set_mag(self.max_speed);
        let mut steering = desired - self.vel;
        steering.limit_mag(self.max_force);
        steering
    }

    pub fn flee(&self, target: Vector) -> Vector {
        -self.seek(target)
    }

    pub fn apply_force(&mut self, force: Vector) {
        self.acc += force
    }

    pub fn update(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
        self.acc *= 0.0;
        self.vel.limit_mag(self.max_speed);
    }

    pub fn edges(&mut self) {
        if self.pos.x < 0.0 {
            self.pos.x = WINDOW_WIDTH as f64;
        }
        if self.pos.x > WINDOW_WIDTH as f64 {
            self.pos.x = 0.0;
        }
        if self.pos.y < 0.0 {
            self.pos.y = WINDOW_HEIGHT as f64;
        }
        if self.pos.y > WINDOW_HEIGHT as f64 {
            self.pos.y = 0.0;
        }
    }

    pub fn show(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        if self.on_screen() {
            gl.draw(args.viewport(), |c, gl| {
                let triangle = [[-self.r, -self.r / 2.0], [-self.r, self.r / 2.0], [self.r, 0.0]];
                polygon(
                    self.color,
                    &triangle,
                    c.transform.trans(self.pos.x, self.pos.y).rot_deg(self.vel.heading()),
                    gl,
                );
            });
        }
    }

    pub fn on_screen(&self) -> bool {
        self.pos.x + self.r > 0.0
            && self.pos.x - self.r <= WINDOW_WIDTH as f64
            && self.pos.y + self.r > 0.0
            && self.pos.y - self.r <= WINDOW_HEIGHT as f64
    }
}
