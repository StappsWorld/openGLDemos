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

    pub fn from_angle<T: 'static + Into<f64> + Copy>(raw_heading: T) -> Vector {
        let heading_deg: f64 = raw_heading.into();
        let heading = heading_deg.to_radians();
        Vector {
            x: heading.cos(),
            y: heading.sin(),
            heading: heading_deg,
            mag: 1.0,
        }
    }

    pub fn random() -> Vector {
        let mut rng = rand::thread_rng();

        let x = rand::Rng::gen_range(&mut rng, -1.0..1.0);
        let y = rand::Rng::gen_range(&mut rng, -1.0..1.0);
        Vector::new(x, y)
    }

    pub fn x_y(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn set_x<T: 'static + Into<f64> + Copy + std::convert::From<f64>>(&mut self, raw_x: T) {
        let y = self.y;
        *self = Vector::new(raw_x, y.into());
    }

    pub fn set_y<T: 'static + Into<f64> + Copy + std::convert::From<f64>>(&mut self, raw_y: T) {
        let x = self.x;
        *self = Vector::new(x, raw_y.into());
    }

    // A function that normalizes this vector.
    pub fn normalize(&mut self) {
        self.x /= self.mag;
        self.y /= self.mag;
        self.mag = 1.0;
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
        if self.mag > max * max {
            self.set_mag(max);
        }
    }

    pub fn angle_between(&self, other: &Vector) -> f64 {
        (1.0 / (self.dot(other) / ((self.mag * other.mag).sqrt())).cos()).to_degrees()
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        (self.x * other.x) + (self.y * other.y)
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
impl std::ops::Mul<Vector> for Vector {
    type Output = Vector3d;

    fn mul(self, other: Vector) -> Vector3d {
        Vector3d::new(
            0.0,
            0.0,
            self.mag() * other.mag() * self.angle_between(&other).to_radians().sin(),
        )
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vector3d {
    x: f64,
    y: f64,
    z: f64,
    heading: (f64, f64),
    mag: f64,
}
impl Vector3d {
    pub fn new<T: 'static + Into<f64> + Copy>(x_raw: T, y_raw: T, z_raw: T) -> Vector3d {
        let x: f64 = x_raw.into();
        let y: f64 = y_raw.into();
        let z: f64 = z_raw.into();
        let mag = (x * x + y * y + z * z).sqrt();
        Vector3d {
            x: x,
            y: y,
            z: z,
            heading: (1.0 / (y / x).tan(), 1.0 / (z / mag).cos()),
            mag: mag,
        }
    }

    pub fn from_heading<T: 'static + Into<f64> + Copy>(theta_raw: T, phi_raw: T) -> Vector3d {
        let theta_deg: f64 = theta_raw.into();
        let theta: f64 = theta_deg.to_radians();
        let phi_deg: f64 = phi_raw.into();
        let phi: f64 = phi_deg.to_radians();
        Vector3d {
            x: theta.cos(),
            y: theta.sin(),
            z: phi.cos(),
            heading: (theta, phi),
            mag: 1.0,
        }
    }

    pub fn random() -> Vector3d {
        let mut rng = rand::thread_rng();

        let x: f64 = rand::Rng::gen_range(&mut rng, -1.0..1.0);
        let y: f64 = rand::Rng::gen_range(&mut rng, -1.0..1.0);
        let z: f64 = rand::Rng::gen_range(&mut rng, -1.0..1.0);
        Vector3d::new(x, y, z)
    }

    pub fn x_y_z(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn set_x<T: 'static + Into<f64> + Copy>(&mut self, x_raw: f64) {
        let x: f64 = x_raw.into();
        *self = Vector3d::new(x, self.y, self.z);
    }

    pub fn set_y<T: 'static + Into<f64> + Copy>(&mut self, y_raw: f64) {
        let y: f64 = y_raw.into();
        *self = Vector3d::new(self.x, y, self.z);
    }

    pub fn set_z<T: 'static + Into<f64> + Copy>(&mut self, z_raw: f64) {
        let z: f64 = z_raw.into();
        *self = Vector3d::new(self.x, self.y, z);
    }

    pub fn set_theta<T: 'static + Into<f64> + Copy>(&mut self, theta_raw: f64) {
        self.set_heading(theta_raw, self.heading.1);
    }

    pub fn set_phi<T: 'static + Into<f64> + Copy>(&mut self, phi_raw: f64) {
        self.set_heading(self.heading.0, phi_raw);
    }

    pub fn set_heading<T: 'static + Into<f64> + Copy>(&mut self, theta_raw: T, phi_raw: T) {
        *self = Vector3d::from_heading(theta_raw, phi_raw) * self.mag;
    }

    // A function that normalizes this vector.
    pub fn normalize(&mut self) {
        self.x /= self.mag;
        self.y /= self.mag;
        self.z /= self.mag;
        self.mag = 1.0;
    }

    // A function that finds the distance between this vector and another.
    pub fn distance(self, other: Vector3d) -> f64 {
        (self - other).mag
    }

    pub fn set_mag(&mut self, mag: f64) {
        self.normalize();
        *self *= mag;
    }

    pub fn mag(&self) -> f64 {
        self.mag
    }

    pub fn heading(&self) -> (f64, f64) {
        self.heading
    }

    pub fn limit_mag(&mut self, max: f64) {
        if self.mag > max * max {
            self.set_mag(max);
        }
    }

    pub fn angle_between(&self, other: &Vector3d) -> f64 {
        (1.0 / (self.dot(other) / ((self.mag * other.mag).sqrt())).cos()).to_degrees()
    }

    pub fn dot(&self, other: &Vector3d) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}
impl std::ops::Add<Vector3d> for Vector3d {
    fn add(self, other: Vector3d) -> Vector3d {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Vector3d::new(x, y, z)
    }

    type Output = Vector3d;
}
impl std::ops::Sub<Vector3d> for Vector3d {
    fn sub(self, other: Vector3d) -> Vector3d {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Vector3d::new(x, y, z)
    }

    type Output = Vector3d;
}
impl std::ops::AddAssign for Vector3d {
    fn add_assign(&mut self, other: Vector3d) {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        *self = Vector3d::new(x, y, z);
    }
}
impl std::ops::SubAssign for Vector3d {
    fn sub_assign(&mut self, other: Vector3d) {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        *self = Vector3d::new(x, y, z);
    }
}
impl std::ops::Mul<f64> for Vector3d {
    type Output = Vector3d;

    fn mul(self, other: f64) -> Vector3d {
        let x = self.x * other;
        let y = self.y * other;
        let z = self.z * other;
        Vector3d::new(x, y, z)
    }
}
impl std::ops::Mul<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn mul(self, other: Vector3d) -> Vector3d {
        let x = (self.x * other.z) - (self.z * other.y);
        let y = (self.z * other.x) - (self.x * other.z);
        let z = (self.x * other.y) - (self.y * other.x);
        Vector3d::new(x, y, z)
    }
}
impl std::ops::MulAssign<f64> for Vector3d {
    fn mul_assign(&mut self, rhs: f64) {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        *self = Vector3d::new(x, y, z);
    }
}
impl std::ops::Neg for Vector3d {
    type Output = Vector3d;
    fn neg(self) -> Vector3d {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        Vector3d::new(x, y, z)
    }
}
impl From<Vector> for Vector3d {
    fn from(vec: Vector) -> Vector3d {
        Vector3d::new(vec.x, vec.y, 0.0)
    }
}
