use crate::{iter, scl, vector::Vector, IX, N, NX, NY};
use graphics::color::WHITE;
use opengl_graphics::{GlGraphics, Texture};
use piston::RenderArgs;

pub struct Fluid {
    pub size: i32,
    pub dt: f64,
    pub diff: f64,
    pub visc: f64,
    pub s: Vec<f64>,
    pub density: Vec<f64>,
    pub v_x: Vec<f64>,
    pub v_y: Vec<f64>,
    pub v_x0: Vec<f64>,
    pub v_y0: Vec<f64>,
}
impl Fluid {
    pub fn new(diff: i32, visc: f64, dt: f64) -> Fluid {
        Fluid {
            size: N as i32,
            dt: dt,
            diff: diff as f64,
            visc: visc,
            s: vec![0.0; (N * N) as usize],
            density: vec![0.0; (N * N) as usize],
            v_x: vec![0.0; (N * N) as usize],
            v_y: vec![0.0; (N * N) as usize],
            v_x0: vec![0.0; (N * N) as usize],
            v_y0: vec![0.0; (N * N) as usize],
        }
    }

    pub fn step(&mut self) {
        self.v_x0 = diffuse(1, self.v_x0.clone(), self.v_x.clone(), self.visc, self.dt);
        self.v_y0 = diffuse(2, self.v_y0.clone(), self.v_y.clone(), self.visc, self.dt);

        let (self_v_x0, self_v_y0, self_v_x, self_v_y) = project(
            self.v_x0.clone(),
            self.v_y0.clone(),
            self.v_x.clone(),
            self.v_y.clone(),
        );
        self.v_x0 = self_v_x0;
        self.v_y0 = self_v_y0;
        self.v_x = self_v_x;
        self.v_y = self_v_y;

        let (self_v_x, self_v_x0, _, self_v_y0) = advect(
            1,
            self.v_x.clone(),
            self.v_x0.clone(),
            None,
            Some(self.v_y0.clone()),
            self.dt,
            Some(true),
        );

        self.v_x = self_v_x;
        self.v_x0 = self_v_x0;
        self.v_y0 = self_v_y0.unwrap();

        let (self_v_y, self_v_y0, self_v_x0, _) = advect(
            2,
            self.v_y.clone(),
            self.v_y0.clone(),
            Some(self.v_x0.clone()),
            None,
            self.dt,
            Some(false),
        );

        self.v_y = self_v_y;
        self.v_y0 = self_v_y0;
        self.v_x0 = self_v_x0.unwrap();

        let (self_v_x, self_v_y, self_v_x0, self_v_y0) = project(
            self.v_x.clone(),
            self.v_y.clone(),
            self.v_x0.clone(),
            self.v_y0.clone(),
        );

        self.v_x = self_v_x;
        self.v_y = self_v_y;
        self.v_x0 = self_v_x0;
        self.v_y0 = self_v_y0;

        self.s = diffuse(0, self.s.clone(), self.density.clone(), self.diff, self.dt);
        let (self_density, self_s, self_v_x, self_v_y) = advect(
            0,
            self.density.clone(),
            self.s.clone(),
            Some(self.v_x.clone()),
            Some(self.v_y.clone()),
            self.dt,
            None,
        );
        self.density = self_density;
        self.s = self_s;
        self.v_x = self_v_x.unwrap();
        self.v_y = self_v_y.unwrap();
    }

    pub fn add_density(&mut self, x: u32, y: u32, amount: f64) {
        self.density[IX(x, y)] += amount;
    }

    pub fn add_velocity(&mut self, x: u32, y: u32, amount_x: f64, amount_y: f64) {
        let index = IX(x, y);
        self.v_x[index] += amount_x;
        self.v_y[index] += amount_y;
    }

    pub fn renderV(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        for i in 0..N {
            for j in 0..N {
                let x = i * scl as u32;
                let y = j * scl as u32;
                let d = self.density[IX(i, j)];

                let vx = self.v_x[IX(i, j)];
                let vy = self.v_y[IX(i, j)];
                if vx + vy > 0.05 {
                    gl.draw(args.viewport(), |c, gl| {
                        line(
                            WHITE,
                            1.0,
                            [
                                x as f64,
                                y as f64,
                                (x as f64 + scl as f64 * vx),
                                (y as f64 + scl as f64 * vy),
                            ],
                            c.transform,
                            gl,
                        );
                    })
                }
            }
        }
    }

    pub fn renderD(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        for j in 0..NY {
            for i in 0..NX {
                let x = i * scl;
                let y = j * scl;
                gl.draw(args.viewport(), |c, gl| {
                    rectangle(
                        [1.0, 1.0, 1.0, self.density[IX(i as u32, j as u32)] as f32],
                        [x as f64, y as f64, scl as f64, scl as f64],
                        c.transform,
                        gl,
                    );
                })
            }
        }
    }

    pub fn fadeD(&mut self) {
        for i in 0..self.density.len() {
            let mut d = self.density[i];
            d -= 0.02;
            if d < 0.0 {
                d = 0.0;
            }
            self.density[i] = d;
        }
    }
}

pub fn diffuse(b: i32, x: Vec<f64>, x0: Vec<f64>, diff: f64, dt: f64) -> Vec<f64> {
    let a = dt * diff * ((N - 2) * (N - 2)) as f64;
    lin_solve(b, x, x0, a, 1.0 + 6.0 * a)
}

pub fn lin_solve(b: i32, x_raw: Vec<f64>, x0: Vec<f64>, a: f64, c: f64) -> Vec<f64> {
    let cRecip = 1.0 / c;
    let mut x = x_raw.clone();

    for k in 0..iter {
        for j in 1..N - 1 {
            for i in 0..N - 1 {
                x[IX(i, j)] = (x0[IX(i, j)]
                    + a * (x[IX(i + 1, j)] + x[IX(i - 1, j)] + x[IX(i, j + 1)] + x[IX(i, j - 1)]))
                    * cRecip;
            }
        }
        x = set_bnd(b, x);
    }
    x
}

pub fn project(
    velocX_raw: Vec<f64>,
    velocY_raw: Vec<f64>,
    p_raw: Vec<f64>,
    div_raw: Vec<f64>,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut velocX = velocX_raw.clone();
    let mut velocY = velocY_raw.clone();
    let mut p = p_raw.clone();
    let mut div = div_raw.clone();
    for j in 1..N - 1 {
        for i in 0..N - 1 {
            div[IX(i, j)] = -0.5
                * (velocX[IX(i + 1, j)] - velocX[IX(i - 1, j)] + velocY[IX(i, j + 1)]
                    - velocY[IX(i, j - 1)])
                / N as f64;
            p[IX(i, j)] = 0.0;
        }
    }

    div = set_bnd(0, div);
    p = set_bnd(0, p);
    p = lin_solve(0, p, div.clone(), 1.0, 6.0);

    for j in 1..N - 1 {
        for i in 0..N - 1 {
            velocX[IX(i, j)] -= 0.5 * (p[IX(i + 1, j)] - p[IX(i - 1, j)]) * N as f64;
            velocY[IX(i, j)] -= 0.5 * (p[IX(i, j + 1)] - p[IX(i, j - 1)]) * N as f64;
        }
    }
    (set_bnd(1, velocX), set_bnd(2, velocY), p, div)
}

pub fn advect(
    b: i32,
    d_raw: Vec<f64>,
    d0_raw: Vec<f64>,
    velocX_raw: Option<Vec<f64>>,
    velocY_raw: Option<Vec<f64>>,
    dt: f64,
    use_x: Option<bool>,
) -> (Vec<f64>, Vec<f64>, Option<Vec<f64>>, Option<Vec<f64>>) {
    let mut d = d_raw.clone();
    let mut d0 = d0_raw.clone();
    let mut velocX = velocX_raw.clone();
    let mut velocY = velocY_raw.clone();

    let mut i0;
    let mut i1;
    let mut j0;
    let mut j1;

    let dtx = dt * (N - 2) as f64;
    let dty = dt * (N - 2) as f64;

    let mut s0;
    let mut s1;
    let mut t0;
    let mut t1;
    let mut tmp1;
    let mut tmp2;
    let mut x;
    let mut y;

    let Nfloat = N as f64;

    let mut j = 1;
    let mut jfloat = 1;

    let (mut usable_vx, mut usable_vy, mut usable_d) = if let Some(use_x) = use_x {
        if use_x {
            (d, velocY.unwrap(), None)
        } else {
            (velocX.unwrap(), d, None)
        }
    } else {
        (velocX.unwrap(), velocY.unwrap(), Some(d))
    };

    while j < N - 1 {
        let mut ifloat = 1;
        let mut i = 1;
        while i < N - 1 {
            tmp1 = dtx * usable_vx[IX(i, j)];
            tmp2 = dty * usable_vy[IX(i, j)];
            x = ifloat as f64 - tmp1;
            y = jfloat as f64 - tmp2;

            if x < 0.5 {
                x = 0.5;
            }
            if x > Nfloat + 0.5 {
                x = Nfloat + 0.5;
            }
            i0 = x.floor();
            i1 = i0 + 1.0;
            if y < 0.5 {
                y = 0.5;
            }
            if y > Nfloat + 0.5 {
                y = Nfloat + 0.5
            };
            j0 = y.floor();
            j1 = j0 + 1.0;

            s1 = x - i0;
            s0 = 1.0 - s1;
            t1 = y - j0;
            t0 = 1.0 - t1;

            let i0i = i0.floor() as u32;
            let i1i = i1.floor() as u32;
            let j0i = j0.floor() as u32;
            let j1i = j1.floor() as u32;

            // DOUBLE CHECK THIS!!!
            match &mut usable_d {
                Some(d) => {
                    if IX(i, j) > d.len()
                        || IX(i0i, j0i) > d0.len()
                        || IX(i0i, j1i) > d0.len()
                        || IX(i1i, j0i) > d0.len()
                        || IX(i1i, j1i) > d0.len()
                    {
                        continue;
                    }
                    d[IX(i, j)] = s0 * (t0 * d0[IX(i0i, j0i)] + t1 * d0[IX(i0i, j1i)])
                        + s1 * (t0 * d0[IX(i1i, j0i)] + t1 * d0[IX(i1i, j1i)])
                }
                None => {
                    if let Some(use_x) = use_x {
                        if use_x {
                            if IX(i, j) > usable_vx.len()
                                || IX(i0i, j0i) > d0.len()
                                || IX(i0i, j1i) > d0.len()
                                || IX(i1i, j0i) > d0.len()
                                || IX(i1i, j1i) > d0.len()
                            {
                                continue;
                            }
                            usable_vx[IX(i, j)] = s0
                                * (t0 * d0[IX(i0i, j0i)] + t1 * d0[IX(i0i, j1i)])
                                + s1 * (t0 * d0[IX(i1i, j0i)] + t1 * d0[IX(i1i, j1i)])
                        } else {
                            if IX(i, j) > usable_vy.len()
                                || IX(i0i, j0i) > d0.len()
                                || IX(i0i, j1i) > d0.len()
                                || IX(i1i, j0i) > d0.len()
                                || IX(i1i, j1i) > d0.len()
                            {
                                continue;
                            }
                            usable_vy[IX(i, j)] = s0
                                * (t0 * d0[IX(i0i, j0i)] + t1 * d0[IX(i0i, j1i)])
                                + s1 * (t0 * d0[IX(i1i, j0i)] + t1 * d0[IX(i1i, j1i)])
                        }
                    } else {
                        unreachable!()
                    }
                }
            }
            i += 1;
            ifloat += 1;
        }

        j += 1;
        jfloat += 1;
    }

    match usable_d {
        Some(d) => (
            set_bnd(b, d).clone(),
            d0,
            Some(usable_vx.clone()),
            Some(usable_vy.clone()),
        ),
        None => {
            if let Some(use_x) = use_x {
                if use_x {
                    let new_vx = set_bnd(b, usable_vx);
                    (
                        new_vx.clone(),
                        d0,
                        Some(new_vx.clone()),
                        Some(usable_vy.clone()),
                    )
                } else {
                    let new_vy = set_bnd(b, usable_vy);
                    (
                        new_vy.clone(),
                        d0,
                        Some(usable_vx.clone()),
                        Some(new_vy.clone()),
                    )
                }
            } else {
                unreachable!()
            }
        }
    }
}

pub fn set_bnd(b: i32, x_raw: Vec<f64>) -> Vec<f64> {
    let mut x = x_raw.clone();
    for i in 1..N - 1 {
        x[IX(i, 0)] = if b == 2 { -x[IX(i, 1)] } else { x[IX(i, 1)] };
        x[IX(i, N - 1)] = if b == 2 {
            -x[IX(i, N - 2)]
        } else {
            x[IX(i, N - 2)]
        };
    }

    for j in 1..N - 1 {
        x[IX(0, j)] = if b == 1 { -x[IX(1, j)] } else { x[IX(1, j)] };
        x[IX(N - 1, j)] = if b == 1 {
            -x[IX(N - 2, j)]
        } else {
            x[IX(N - 2, j)]
        };
    }

    x[IX(0, 0)] = 0.5 * (x[IX(1, 0)] + x[IX(0, 1)]);
    x[IX(0, N - 1)] = 0.5 * (x[IX(1, N - 1)] + x[IX(0, N - 2)]);
    x[IX(N - 1, 0)] = 0.5 * (x[IX(N - 2, 0)] + x[IX(N - 1, 1)]);
    x[IX(N - 1, N - 1)] = 0.5 * (x[IX(N - 2, N - 1)] + x[IX(N - 1, N - 2)]);
    x
}
