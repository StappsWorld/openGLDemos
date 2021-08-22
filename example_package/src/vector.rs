#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vector {
    x: f64,
    y: f64,
    heading: f64,
    mag: f64,
}
impl Vector {
    pub fn new<T: 'static + Into<f64> + Copy>(raw_x: T, raw_y: T) -> Vector {
        let x: f64 = raw_x.into();
        let y: f64 = raw_y.into();
        Vector {
            x: x,
            y: y,
            heading: ang::atan2(y, x).in_degrees(),
            mag: (x * x + y * y).sqrt(),
        }
    }

    pub fn x_y(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn set_x<T: 'static + Into<f64> + Copy + std::convert::From<f64>>(&mut self, raw_x : T) {
        let y = self.y;
        *self = Vector::new(raw_x, y.into());
    }

    pub fn set_y<T: 'static + Into<f64> + Copy + std::convert::From<f64>>(&mut self, raw_y : T) {
        let x = self.x;
        *self = Vector::new(x, raw_y.into());
    }

    // A function that normalizes this vector.
    pub fn normalize(&mut self) {
        let len = ((self.x * self.x) + (self.y * self.y)).sqrt();
        self.x /= len;
        self.y /= len;
    }

    // A function that finds the distance between this vector and another.
    pub fn distance(&mut self, other: &Vector) -> f64 {
        (*self - *other).mag
    }

    pub fn set_mag(&mut self, mag: f64) {
        self.normalize();
        self.x *= mag;
        self.y *= mag;
        self.mag = mag;
    }

    // A function that calculates the magnitude of this vector.
    pub fn mag(&self) -> f64 {
        self.mag
    }

    pub fn heading(&self) -> f64 {
        self.heading
    }

    pub fn set_heading(&mut self, heading: f64) {
        let radians = heading.to_radians();

        self.x = radians.cos();
        self.y = radians.sin();
        self.heading = heading;
    }

    pub fn limit_mag(&mut self, max: f64) {
        if (self.x * self.x + self.y * self.y) > max * max {
            self.set_mag(max);
        }
    }

    pub fn random2D() -> Vector {
        let mut rng = rand::thread_rng();

        let x = rand::Rng::gen_range(&mut rng, -1.0..1.0);
        let y = rand::Rng::gen_range(&mut rng, -1.0..1.0);
        Vector {
            x: x,
            y: y,
            heading: ang::atan2(y, x).in_degrees(),
            mag: (x * x + y * y).sqrt(),
        }
    }
}
impl std::ops::Add<Vector> for Vector {
    fn add(self, other: Vector) -> Vector {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vector::new(x, y)
    }

    type Output = Vector;
}
impl std::ops::Sub<Vector> for Vector {
    fn sub(self, other: Vector) -> Vector {
        let x = self.x - other.x;
        let y = self.y - other.y;
        Vector::new(x, y)
    }

    type Output = Vector;
}
impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        let x = self.x + other.x;
        let y = self.y + other.y;
        *self = Vector::new(x, y);
    }
}
impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        let x = self.x - other.x;
        let y = self.y - other.y;
        *self = Vector::new(x, y);
    }
}
impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        let x = self.x * other;
        let y = self.y * other;
        Vector::new(x, y)
    }
}
impl std::ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        let x = self.x * rhs;
        let y = self.y * rhs;
        *self = Vector::new(x, y);
    }
}
impl std::ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        let x = -self.x;
        let y = -self.y;
        Vector::new(x, y)
    }
}
