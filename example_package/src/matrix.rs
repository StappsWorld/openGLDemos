use crate::vector::Vector;

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}
impl Matrix {
    pub fn new_empty<T: 'static + Into<usize> + Copy>(rows_raw: T, cols_raw: T) -> Matrix {
        let rows = rows_raw.into();
        let cols = cols_raw.into();
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn new<T: 'static + Into<usize> + Copy>(
        rows_raw: T,
        cols_raw: T,
        data: Vec<f64>,
    ) -> Matrix {
        let rows = rows_raw.into();
        let cols = cols_raw.into();
        Matrix { rows, cols, data }
    }

    pub fn identity<T: 'static + Into<usize> + Copy>(size_raw: T) -> Matrix {
        let size = size_raw.into();
        let mut m = Matrix::new_empty(size, size);
        for i in 0..size {
            m.set(i, i, 1.0);
        }
        m
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn data(&self) -> &Vec<f64> {
        &self.data
    }

    pub fn get<T: 'static + Into<usize> + Copy>(&self, row: T, col: T) -> Option<f64> {
        let row = row.into();
        let col = col.into();
        if row >= self.rows || col >= self.cols {
            return None;
        }

        Some(self.data[row * self.cols + col])
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Option<()> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        self.data[row * self.cols + col] = value;
        Some(())
    }

    pub fn dot(&self, other: &Matrix) -> Option<Matrix> {
        if self.cols != other.rows {
            return None;
        }
        let mut result = Matrix::new_empty(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * other.data[k * other.cols + j];
                }
                result.data[i * other.cols + j] = sum;
            }
        }
        Some(result)
    }

    pub fn determinant(&self) -> f64 {
        if self.rows != self.cols {
            return 0.0;
        }
        if self.rows == 1 {
            return self.data[0];
        }
        if self.rows == 2 {
            return self.data[0] * self.data[3] - self.data[1] * self.data[2];
        }
        let mut sum = 0.0;
        for i in 0..self.cols {
            sum += self.data[i] * self.cofactor(0, i);
        }
        sum
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let mut m = Matrix::new_empty(self.rows - 1, self.cols - 1);
        let mut i = 0;
        let mut j = 0;
        for r in 0..self.rows {
            if r == row {
                continue;
            }
            for c in 0..self.cols {
                if c == col {
                    continue;
                }
                m.data[i * (m.cols - 1) + j] = self.data[r * self.cols + c];
                j += 1;
            }
            i += 1;
            j = 0;
        }
        m.determinant() * (if (row + col) % 2 == 0 { 1.0 } else { -1.0 })
    }

    pub fn eigenvalues_eigenvectors(&self) -> Option<Vec<(Vector, f64)>> {
        if self.rows != self.cols {
            return None;
        }
        
        unimplemented!()
    }
}
impl std::ops::Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Matrix {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|x| x * rhs).collect(),
        }
    }
}
impl std::ops::MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..self.data.len() {
            self.data[i] *= rhs;
        }
    }
}
impl std::ops::Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Matrix dimensions do not match");
        }
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(x, y)| x + y)
                .collect(),
        }
    }
}
impl std::ops::AddAssign<Matrix> for Matrix {
    fn add_assign(&mut self, rhs: Matrix) {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Matrix dimensions do not match");
        }
        for i in 0..self.data.len() {
            self.data[i] += rhs.data[i];
        }
    }
}
impl std::ops::Sub<Matrix> for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix {
        self + -rhs
    }
}
impl std::ops::SubAssign<Matrix> for Matrix {
    fn sub_assign(&mut self, rhs: Matrix) {
        *self += -rhs
    }
}
impl std::ops::Neg for Matrix {
    type Output = Matrix;
    fn neg(self) -> Matrix {
        self * -1.0
    }
}
