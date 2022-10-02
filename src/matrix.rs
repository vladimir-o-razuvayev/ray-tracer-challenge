use crate::tuple::Tuple;

#[derive(Debug)]
pub(crate) struct Matrix {
    width: usize,
    height: usize,
    data: Vec<Tuple>,
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            width,
            height,
            data: vec![Tuple::zero(); width * height],
        }
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = Tuple;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Tuple {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, point::Point, tuple::Tuple, vector::Vector};

    #[test]
    fn access_matrix_by_index() {
        let tuple0 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let tuple1 = Tuple::new(5.5, 6.5, 7.5, 8.5);
        let tuple2 = Tuple::new(9.0, 10.0, 11.0, 12.0);
        let tuple3 = Tuple::new(13.5, 14.5, 15.5, 16.5);
        let mut matrix = Matrix::new(4, 4);

        matrix[0] = tuple0;
        matrix[1] = tuple1;
        matrix[2] = tuple2;
        matrix[3] = tuple3;

        assert_eq!(matrix[0][1], 2.0);
        assert_eq!(matrix[1][0], 5.5);
        assert_eq!(matrix[2][2], 11.0);
        assert_eq!(matrix[3][3], 16.5);
    }
}
