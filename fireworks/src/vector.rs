

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
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
} impl std::ops::Mul<f64> for Vector {
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