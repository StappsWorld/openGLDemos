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
        self.cos_angle_between(other).acos().to_degrees()
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn cos_angle_between(&self, other: &Vector) -> f64 {
        self.dot(other) / (self.mag() * other.mag())
    }

    // This function returns the vector projection of self onto other
    pub fn project(&self, other: &Vector) -> Vector {
        let dot = self.dot(other);
        let mag = other.mag * other.mag;
        let frac = dot / mag;
        *other * frac
    }

    // This function returns the scalar component of self in the direction of other
    pub fn scalar_comp(&self, other: &Vector) -> f64 {
        self.mag() * self.cos_angle_between(other)
    }

    // Functionally the same as self * other
    pub fn cross(self, other: Vector) -> Vector3d {
        self * other
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
impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        self * (1.0 / other)
    }
}
impl std::ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        *self *= (1.0 / rhs);
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
impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
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

    // This function generates a Vector3d from a theta (angle from x axis to [v.x, v.y]) and phi (angle from y axis [v.y, v.z]) in degrees.
    pub fn from_heading<T: 'static + Into<f64> + Copy + std::convert::From<f64>>(theta_raw: T, phi_raw: T) -> Vector3d {
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

    pub fn set_x<T: 'static + Into<f64> + Copy + std::convert::From<f64>>(&mut self, x_raw: T) {
        let x: f64 = x_raw.into();
        *self = Vector3d::new(x, self.y, self.z);
    }

    pub fn set_y<T: 'static + Into<f64> + Copy + std::convert::From<f64>>(&mut self, y_raw: T) {
        let y: f64 = y_raw.into();
        *self = Vector3d::new(self.x, y, self.z);
    }

    pub fn set_z<T: 'static + Into<f64> + Copy + std::convert::From<f64>>(&mut self, z_raw: T) {
        let z: f64 = z_raw.into();
        *self = Vector3d::new(self.x, self.y, z);
    }

    // This function sets theta (angle from x axis to [v.x, v.y]) in degrees.
    pub fn set_theta<T: 'static + Into<f64> + Copy + std::convert::From<f64>>(&mut self, theta_raw: T) {
        self.set_heading(theta_raw, self.heading.1.into());
    }

    // This function sets phi (angle from y axis [v.y, v.z]) in degrees.
    pub fn set_phi<T: 'static + Into<f64> + Copy + std::convert::From<f64>>(&mut self, phi_raw: T) {
        self.set_heading(self.heading.0, phi_raw.into());
    }

    // This function sets the heading in (theta, phi) format in degrees.
    pub fn set_heading<T: 'static + Into<f64> + Copy + std::convert::From<f64>>(&mut self, theta_raw: T, phi_raw: T) {
        *self = Vector3d::from_heading(theta_raw, phi_raw) * self.mag;
    }

    // A function that normalizes this vector.
    pub fn normalize(&mut self) {
        self.x /= self.mag;
        self.y /= self.mag;
        self.z /= self.mag;
        self.mag = 1.0;
    }

    pub fn get_normalized(&self) -> Vector3d {
        let mut v = self.clone();
        v.normalize();
        v
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

    // This angle returns the angle between two Vector3d's in degrees.
    pub fn angle_between(&self, other: &Vector3d) -> f64 {
        (1.0 / (self.dot(other) / ((self.mag * other.mag).sqrt())).cos()).to_degrees()
    }

    pub fn cos_angle_between(&self, other: &Vector3d) -> f64 {
        self.dot(other) / (self.mag * other.mag)
    }

    pub fn dot(&self, other: &Vector3d) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    // This function returns the vector projection of self onto other
    pub fn project(&self, other: &Vector3d) -> Vector3d {
        let dot = self.dot(other);
        let mag = other.mag * other.mag;
        let frac = dot / mag;
        *other * frac
    }

    // This function returns the scalar component of self in the direction of other
    pub fn scalar_comp(&self, other: &Vector3d) -> f64 {
        self.mag() * self.cos_angle_between(other)
    }

    // Functionally the same as self * other
    pub fn cross(self, other: Vector3d) -> Vector3d {
        self * other
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
        let x = (self.y * other.z) - (self.z * other.y);
        let y = (self.z * other.x) - (self.x * other.z);
        let z = (self.x * other.y) - (self.y * other.x);
        Vector3d::new(x, y, z)
    }
}
impl std::ops::Div<f64> for Vector3d {
    type Output = Vector3d;

    fn div(self, other: f64) -> Vector3d {
        self * (1.0 / other)
    }
}
impl std::ops::DivAssign<f64> for Vector3d {
    fn div_assign(&mut self, other: f64) {
        *self *= (1.0 / other);
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
impl std::fmt::Display for Vector3d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct VectorN {
    data: Vec<f64>,
    mag : f64,
} impl VectorN {
    pub fn new(data: Vec<f64>) -> VectorN {
        let mut v = VectorN { data, mag : 0.0 };
        v.update_mag();
        v
    }

    pub fn new_empty(size: usize) -> VectorN {
        VectorN {
            data: vec![0.0; size],
            mag: 0.0,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> f64 {
        self.data[index]
    }

    pub fn set<T: 'static + Into<f64> + Copy>(&mut self, index: usize, val: T) -> Option<()> {
        if index >= self.data.len() {
            return None;
        }
        self.data[index] = val.into();
        Some(())
    }

    pub fn dot(&self, other: &VectorN) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.len() {
            sum += self.data[i] * other.data[i];
        }
        sum
    }

    pub fn mag(&self) -> f64 {
        self.mag
    }

    pub fn update_mag(&mut self) {
        self.mag = self.dot(&self).sqrt();
    }

    pub fn normalize(&mut self) {
        let mag = self.mag();
        for i in 0..self.len() {
            self.data[i] /= mag;
        }
    }

    pub fn cos_angle_between(&self, other: &VectorN) -> f64 {
        self.dot(other) / (self.mag() * other.mag())
    }

    // This function returns the vector projection of self onto other
    pub fn project(&self, other: &VectorN) -> VectorN {
        let dot = self.dot(other);
        let mag = other.mag * other.mag;
        let frac = dot / mag;
        other.clone() * frac
    }

    // This function returns the scalar component of self in the direction of other
    pub fn scalar_comp(&self, other: &VectorN) -> f64 {
        self.mag() * self.cos_angle_between(other)
    }

    // TODO - Implement cross product
}
impl std::ops::Add<VectorN> for VectorN {
    fn add(self, other: VectorN) -> VectorN {
        let data = self.data.iter().zip(other.data.iter()).map(|(a, b)| a + b).collect();
        VectorN::new(data)
    }

    type Output = VectorN;
}
impl std::ops::Sub<VectorN> for VectorN {
    fn sub(self, other: VectorN) -> VectorN {
        let data = self.data.iter().zip(other.data.iter()).map(|(a, b)| a - b).collect();
        VectorN::new(data)
    }

    type Output = VectorN;
}
impl std::ops::AddAssign for VectorN {
    fn add_assign(&mut self, other: VectorN) {
        *self = self.clone() + other;
    }
}
impl std::ops::SubAssign for VectorN {
    fn sub_assign(&mut self, other: VectorN) {
        *self = self.clone() + other;
    }
}
impl std::ops::Mul<f64> for VectorN {
    type Output = VectorN;

    fn mul(self, rhs: f64) -> VectorN {
        let data = self.data.iter().map(|a| a * rhs).collect();
        VectorN::new(data)
    }
}
impl std::ops::MulAssign<f64> for VectorN {
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.clone() * rhs;
    }
}
impl std::ops::Div<f64> for VectorN {
    type Output = VectorN;

    fn div(self, rhs: f64) -> VectorN {
        self * (1.0 / rhs)
    }
}
impl std::ops::DivAssign<f64> for VectorN {
    fn div_assign(&mut self, rhs: f64) {
        *self = self.clone() / rhs;
    }
}
impl std::ops::Neg for VectorN {
    type Output = VectorN;
    fn neg(self) -> VectorN {
        self * -1.0
    }
}
impl From<Vector> for VectorN {
    fn from(vec: Vector) -> VectorN {
        VectorN::new(vec![vec.x, vec.y])
    }
}
impl From<Vector3d> for VectorN {
    fn from(vec: Vector3d) -> VectorN {
        VectorN::new(vec![vec.x, vec.y, vec.z])
    }
}
impl std::fmt::Display for VectorN {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<")?;
        for i in 0..self.len() {
            if i == self.len() - 1 {
                write!(f, "{}", self.data[i])?;
            } else {
                write!(f, "{}, ", self.data[i])?;
            }
        }
        write!(f, ">")
    }
}