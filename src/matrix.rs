use float_cmp::approx_eq;

use crate::{point::Point, tup, tuple::Tuple, vector::Vector};

#[macro_export]
macro_rules! mat {
    () => (
        Matrix::zero()
    );
    ($( $( $x: expr ),*);*) => (
        Matrix::new([ $( [ $($x),* ] ),* ])
    );
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    pub data: [[f32; 4]; 4],
}
impl Matrix {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Matrix { data }
    }

    pub fn zero() -> Self {
        mat![0.0, 0.0, 0.0, 0.0;
             0.0, 0.0, 0.0, 0.0;
             0.0, 0.0, 0.0, 0.0;
             0.0, 0.0, 0.0, 0.0]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Matrix) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if !approx_eq!(f32, self[row][col], rhs[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut matrix = Matrix::zero();
        for row in 0..4 {
            for col in 0..4 {
                matrix[row][col] = self[row][0] * rhs[0][col]
                    + self[row][1] * rhs[1][col]
                    + self[row][2] * rhs[2][col]
                    + self[row][3] * rhs[3][col];
            }
        }
        matrix
    }
}

impl std::ops::Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            (self[0][0] * rhs.x) + (self[0][1] * rhs.y) + (self[0][2] * rhs.z),
            (self[1][0] * rhs.x) + (self[1][1] * rhs.y) + (self[1][2] * rhs.z),
            (self[2][0] * rhs.x) + (self[2][1] * rhs.y) + (self[2][2] * rhs.z),
        )
    }
}

impl std::ops::Mul<Point> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Point) -> Self::Output {
        tup![
            (self[0][0] * rhs.x) + (self[0][1] * rhs.y) + (self[0][2] * rhs.z) + self[0][3],
            (self[1][0] * rhs.x) + (self[1][1] * rhs.y) + (self[1][2] * rhs.z) + self[1][3],
            (self[2][0] * rhs.x) + (self[2][1] * rhs.y) + (self[2][2] * rhs.z) + self[2][3],
            self[3][0] + self[3][1] + self[3][2] + self[3][3]
        ]
    }
}

impl std::ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        tup![
            (self[0][0] * rhs[0])
                + (self[0][1] * rhs[1])
                + (self[0][2] * rhs[2])
                + (self[0][3] * rhs[3]),
            (self[1][0] * rhs[0])
                + (self[1][1] * rhs[1])
                + (self[1][2] * rhs[2])
                + (self[1][3] * rhs[3]),
            (self[2][0] * rhs[0])
                + (self[2][1] * rhs[1])
                + (self[2][2] * rhs[2])
                + (self[2][3] * rhs[3]),
            (self[3][0] * rhs[0])
                + (self[3][1] * rhs[1])
                + (self[3][2] * rhs[2])
                + (self[2][3] * rhs[3])
        ]
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = [f32; 4];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut [f32; 4] {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {

    use crate::{mat, matrix::Matrix, point::Point, tup, tuple::Tuple, vector::Vector};

    #[test]
    fn two_matrices_approx_eq() {
        let matrix1 = mat![1.0, 2.0, 3.0, 4.0;
                           5.5, 6.5, 7.5, 8.5;
                           9.0, 10.0, 11.0, 12.0;
                           13.5, 14.5, 15.5, 16.5];

        let matrix2 = mat![0.9999999, 2.0, 3.0, 4.0;
                           5.5, 6.5, 7.5, 8.5;
                           9.0, 10.0, 11.0, 12.0;
                           13.5, 14.5, 15.5, 16.5];

        assert_eq!(matrix1, matrix2)
    }

    #[test]
    fn two_matrices_approx_ne() {
        let matrix1 = mat![1.0, 2.0, 3.0, 4.0;
                           5.5, 6.5, 7.5, 8.5;
                           9.0, 10.0, 11.0, 12.0;
                           13.5, 14.5, 15.5, 16.5];

        let matrix2 = mat![0.999, 2.0, 3.0, 4.0;
                           5.5, 6.5, 7.5, 8.5;
                           9.0, 10.0, 11.0, 12.0;
                           13.5, 14.5, 15.5, 16.5];

        assert_ne!(matrix1, matrix2)
    }
    #[test]
    fn mult_matrix_by_vector() {
        let matrix = mat![1.0, 2.0, 3.0, 4.0;
                          2.0, 4.0, 4.0, 2.0;
                          8.0, 6.0, 4.0, 1.0;
                          0.0, 0.0, 0.0, 1.0];
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(matrix * vector, Vector::new(14.0, 22.0, 32.0));
    }
    #[test]
    fn mult_matrix_by_point() {
        let matrix = mat![1.0, 2.0, 3.0, 4.0;
                          2.0, 4.0, 4.0, 2.0;
                          8.0, 6.0, 4.0, 1.0;
                          0.0, 0.0, 0.0, 1.0];
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(matrix * point, tup![18.0, 24.0, 33.0, 1.0]);
    }
    #[test]
    fn mult_matrix_by_tuple() {
        let matrix = mat![1.0, 2.0, 3.0, 4.0;
                          2.0, 4.0, 4.0, 2.0;
                          8.0, 6.0, 4.0, 1.0;
                          0.0, 0.0, 0.0, 1.0];
        let tuple = tup![1.0, 2.0, 3.0, 1.0];
        assert_eq!(matrix * tuple, tup![18.0, 24.0, 33.0, 1.0]);
    }
    #[test]
    fn access_matrix_by_index() {
        let mut matrix = mat![1.0, 2.0, 3.0, 4.0;
                              5.5, 6.5, 7.5, 8.5;
                              9.0, 10.0, 11.0, 12.0;
                              13.5, 14.5, 15.5, 16.5];

        assert_eq!(matrix[3][3], 16.5);

        matrix[0][1] = 0.0;

        assert_eq!(matrix[0][1], 0.0);
    }
}
