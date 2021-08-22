use glutin_window::GlutinWindow as Window;
use graphics::color::WHITE;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{AdvancedWindow, EventLoop, MouseCursorEvent, RenderArgs};
use rand::Rng;
use Raycasting::FPSCounter;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;
const PI : f64 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Vector {
    pub x: f64,
    pub y: f64,
}
impl Vector {
    // A function that normalizes this vector.
    fn normalize(&mut self) {
        let len = ((self.x * self.x) + (self.y * self.y)).sqrt();
        self.x /= len;
        self.y /= len;
    }

    // A function that finds the distance between this vector and another.
    fn distance(&mut self, other: &Vector) -> f64 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        let diff_sq = (x_diff * x_diff) + (y_diff * y_diff);
        let sqrt_diff = diff_sq.sqrt();
        return sqrt_diff;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Boundary {
    pub a: Vector,
    pub b: Vector,
}
impl Boundary {
    pub fn show(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
        gl.draw(args.viewport(), |context, gl| {
            line(
                WHITE,
                1.0,
                [self.a.x, self.a.y, self.b.x, self.b.y],
                context.transform,
                gl,
            )
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Ray {
    pub pos: Vector,
    pub dir: Vector,
}
impl Ray {
    pub fn new(pos: [f64; 2], angle: f64) -> Ray {
        Ray {
            pos: Vector {
                x: pos[0],
                y: pos[1],
            },
            dir: Vector {
                x: angle.cos(),
                y: angle.sin(),
            },
        }
    }

    pub fn lookAt(&mut self, x: f64, y: f64) {
        self.dir.x = x - self.pos.x;
        self.dir.y = y - self.pos.y;
        self.dir.normalize();
    }

    pub fn show(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
        gl.draw(args.viewport(), |context, gl| {
            line(
                WHITE,
                1.0,
                [
                    self.pos.x,
                    self.pos.y,
                    self.pos.x + (self.dir.x * 10.0),
                    self.pos.y + (self.dir.y * 10.0),
                ],
                context.transform,
                gl,
            );
        });
    }

    pub fn cast(&self, wall: &Boundary) -> Option<Vector> {
        let x1: f64 = wall.a.x;
        let y1: f64 = wall.a.y;
        let x2: f64 = wall.b.x;
        let y2: f64 = wall.b.y;

        let x3: f64 = self.pos.x;
        let y3: f64 = self.pos.y;
        let x4: f64 = self.pos.x + self.dir.x;
        let y4: f64 = self.pos.y + self.dir.y;

        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if den == 0.0 {
            return None;
        }

        let num1 = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
        let num2 = -1.0 * ((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3));

        let t = num1 / den;
        let u = num2 / den;

        if t > 0.0 && t < 1.0 && u > 0.0 {
            Some(Vector {
                x: x1 + t * (x2 - x1),
                y: y1 + t * (y2 - y1),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Particle {
    pub pos: Vector,
    pub rays: Vec<Ray>,
}
impl Particle {
    pub fn new() -> Particle {
        let pos: Vector = Vector {
            x: (WINDOW_WIDTH as f64 / 2.0),
            y: (WINDOW_HEIGHT as f64 / 2.0),
        };

        let mut rays = vec![];
        for i_raw in 0..360 {
            let i = i_raw as f64;
            rays.push(Ray {
                pos: pos,
                dir: Vector {
                    x: (i * (1.0 / 180.0) * PI).cos(),
                    y: (i * (1.0 / 180.0) * PI).sin(),
                },
            });
        }
        Particle {
            pos: pos,
            rays: rays,
        }
    }

    pub fn show(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        gl.draw(args.viewport(), |_context, gl| {
            for ray in self.rays.clone() {
                ray.show(gl, args);
            }
        });
    }

    pub fn look(&mut self, walls: Vec<Boundary>, step : usize, gl: &mut GlGraphics, args: &RenderArgs) -> usize {
        use graphics::*;

        let mut total_rays = 0;
        for ray in self.rays.clone().iter().step_by(step) {
            let mut record: (f64, Vector) = (f64::MAX, Vector { x: 0.0, y: 0.0 });

            for wall in walls.clone() {
                if let Some(pt) = ray.cast(&wall) {
                    let dist = self.pos.distance(&pt);

                    if dist < record.0 {
                        record = (dist, pt);
                    }
                }
            }

            if record.0 != f64::MAX {
                total_rays += 1;
                gl.draw(args.viewport(), |context, gl| {
                    line(
                        [1.0, 1.0, 1.0, 0.7],
                        0.5,
                        [self.pos.x, self.pos.y, record.1.x, record.1.y],
                        context.transform,
                        gl,
                    );
                });
            }
        }
        total_rays
    }

    pub fn update(&mut self, pos: [f64; 2]) {
        self.pos.x = pos[0];
        self.pos.y = pos[1];
        let mut new_rays = vec![];
        for ray in self.rays.clone() {
            let mut new_ray = ray;
            new_ray.pos = self.pos;
            new_rays.push(new_ray);
        }
        self.rays = new_rays;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Raycasting Test", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut fps_counter: FPSCounter = FPSCounter::new();

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(10000);

    let mut rng = rand::thread_rng();

    let bounds = vec![
        Boundary {
            a: Vector { x: 0.0, y: 0.0 },
            b: Vector {
                x: WINDOW_WIDTH as f64,
                y: 0.0,
            },
        },
        Boundary {
            a: Vector {
                x: WINDOW_WIDTH as f64,
                y: 0.0,
            },
            b: Vector {
                x: WINDOW_WIDTH as f64,
                y: WINDOW_HEIGHT as f64,
            },
        },
        Boundary {
            a: Vector {
                x: WINDOW_WIDTH as f64,
                y: WINDOW_HEIGHT as f64,
            },
            b: Vector {
                x: 0.0,
                y: WINDOW_HEIGHT as f64,
            },
        },
        Boundary {
            a: Vector {
                x: 0.0,
                y: WINDOW_HEIGHT as f64,
            },
            b: Vector { x: 0.0, y: 0.0 },
        },
    ];

    let mut walls = vec![];

    for _ in 0..10 {
        walls.push(Boundary {
            a: Vector {
                x: rng.gen_range(0..WINDOW_WIDTH) as f64,
                y: rng.gen_range(0..WINDOW_HEIGHT) as f64,
            },
            b: Vector {
                x: rng.gen_range(0..WINDOW_WIDTH) as f64,
                y: rng.gen_range(0..WINDOW_HEIGHT) as f64,
            },
        });
    }

    for wall in bounds.clone() {
        walls.push(wall);
    }

    let mut step : usize = 5;

    let mut particle = Particle::new();

    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            let mut total_rays = 0;
            gl.draw(args.viewport(), |_context, gl| {
                clear(BLACK, gl);

                for wall in walls.clone() {
                    wall.show(gl, &args);
                }
                total_rays = particle.look(walls.clone(), step, gl, &args);
            });

            window.set_title(format!(
                "Raycasting Test | {:03} fps | {:04} Rays Drawn | {:02} Walls Drawn | Showing Every {:03} Ray",
                fps_counter.tick(),
                total_rays,
                walls.len(),
                step
            ));
        } else if let Some([x, y]) = e.mouse_cursor_args() {
            particle.update([x, y]);
        } else if let piston::Event::Input(i, _) = e {
            match i {
                piston::Input::Button(b) => match (b.state, b.button) {
                    (piston::ButtonState::Release, piston::Button::Keyboard(k)) => match k {
                        piston::Key::Space => {
                            walls = vec![];
                            for _ in 0..rng.gen_range(5..20) {
                                walls.push(Boundary {
                                    a: Vector {
                                        x: rng.gen_range(0..WINDOW_WIDTH) as f64,
                                        y: rng.gen_range(0..WINDOW_HEIGHT) as f64,
                                    },
                                    b: Vector {
                                        x: rng.gen_range(0..WINDOW_WIDTH) as f64,
                                        y: rng.gen_range(0..WINDOW_HEIGHT) as f64,
                                    },
                                });
                            }

                            for wall in bounds.clone() {
                                walls.push(wall);
                            }
                        },
                        piston::Key::A => {
                            step += 1;
                            if step >= 360 {
                                step = 1;
                            }
                        },
                        piston::Key::S => {
                            step -= 1;
                            if step <= 0 {
                                step = 359;
                            }
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
