#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}
impl Vector {
    // A function that normalizes this vector.
    pub fn normalize(&mut self) {
        let len = ((self.x * self.x) + (self.y * self.y)).sqrt();
        self.x /= len;
        self.y /= len;
    }

    // A function that finds the distance between this vector and another.
    pub fn distance(&self, other: &Vector) -> f64 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        let diff_sq = (x_diff * x_diff) + (y_diff * y_diff);
        let sqrt_diff = diff_sq.sqrt();
        return sqrt_diff;
    }

    pub fn set_mag(&mut self, mag: f64) {
        self.normalize();
        self.x *= mag;
        self.y *= mag;
    }

    // A function that calculates the magnitude of this vector.
    pub fn mag(&self) -> f64 {
        let x_diff = self.x;
        let y_diff = self.y;
        let diff_sq = (x_diff * x_diff) + (y_diff * y_diff);
        let sqrt_diff = diff_sq.sqrt();
        return sqrt_diff;
    }

    pub fn heading(&self) -> f64 {
        ang::atan2(self.y, self.x).in_degrees()
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
        Vector { x: x, y: y }
    }
}
impl std::ops::Add<Vector> for Vector {
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    type Output = Vector;
}
impl std::ops::Sub<Vector> for Vector {
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    type Output = Vector;
}
impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}
impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl std::ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl std::ops::Div<f64> for Vector {
    type Output = Vector;
    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
impl std::ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
impl std::ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}
