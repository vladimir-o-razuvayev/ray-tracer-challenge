use crate::tuple::Tuple;

pub(crate) enum MixedTuples {
    TwoTuple(Tuple, Tuple),
    ThreeTuple(Tuple, Tuple, Tuple),
    FourTuple(Tuple, Tuple, Tuple, Tuple),
}

impl From<(Tuple, Tuple)> for MixedTuples {
    fn from((a, b): (Tuple, Tuple)) -> MixedTuples {
        MixedTuples::TwoTuple(a, b)
    }
}

impl From<(Tuple, Tuple, Tuple)> for MixedTuples {
    fn from((a, b, c): (Tuple, Tuple, Tuple)) -> MixedTuples {
        MixedTuples::ThreeTuple(a, b, c)
    }
}

impl From<(Tuple, Tuple, Tuple, Tuple)> for MixedTuples {
    fn from((a, b, c, d): (Tuple, Tuple, Tuple, Tuple)) -> MixedTuples {
        MixedTuples::FourTuple(a, b, c, d)
    }
}

#[derive(Debug)]
pub(crate) struct Matrix {
    width: usize,
    height: usize,
    data: Vec<Tuple>,
}

impl Matrix {
    pub fn new<A>(data: A) -> Self
    where
        A: Into<MixedTuples>,
    {
        match data.into() {
            MixedTuples::TwoTuple(a, b) => Matrix {
                width: 2,
                height: 2,
                data: vec![a, b],
            },
            MixedTuples::ThreeTuple(a, b, c) => Matrix {
                width: 3,
                height: 3,
                data: vec![a, b, c],
            },
            MixedTuples::FourTuple(a, b, c, d) => Matrix {
                width: 4,
                height: 4,
                data: vec![a, b, c, d],
            },
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Matrix) -> bool {
        (self[0] == rhs[0]) && (self[1] == rhs[1]) && (self[2] == rhs[2]) && (self[3] == rhs[3])
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
    fn two_matrices_approx_eq() {
        let matrix1 = Matrix::new((
            Tuple::new((1.0, 2.0, 3.0, 4.0)),
            Tuple::new((5.5, 6.5, 7.5, 8.5)),
            Tuple::new((9.0, 10.0, 11.0, 12.0)),
            Tuple::new((13.5, 14.5, 15.5, 16.5)),
        ));

        let matrix2 = Matrix::new((
            Tuple::new((1.0, 2.0, 3.0, 4.0)) * 0.9999999,
            Tuple::new((5.5, 6.5, 7.5, 8.5)) * 1.0000001,
            Tuple::new((9.0, 10.0, 11.0, 12.0)),
            Tuple::new((13.5, 14.5, 15.5, 16.5)),
        ));

        assert_eq!(matrix1, matrix2)
    }

    #[test]
    fn two_matrices_approx_ne() {
        let matrix1 = Matrix::new((
            Tuple::new((1.0, 2.0, 3.0, 4.0)),
            Tuple::new((5.5, 6.5, 7.5, 8.5)),
            Tuple::new((9.0, 10.0, 11.0, 12.0)),
            Tuple::new((13.5, 14.5, 15.5, 16.5)),
        ));

        let matrix2 = Matrix::new((
            Tuple::new((1.0, 2.0, 3.0, 4.0)) * 0.5,
            Tuple::new((5.5, 6.5, 7.5, 8.5)),
            Tuple::new((9.0, 10.0, 11.0, 12.0)),
            Tuple::new((13.5, 14.5, 15.5, 16.5)),
        ));

        assert_ne!(matrix1, matrix2)
    }

    #[test]
    fn access_matrix_by_index() {
        let mut matrix = Matrix::new((
            Tuple::new((1.0, 2.0, 3.0, 4.0)),
            Tuple::new((5.5, 6.5, 7.5, 8.5)),
            Tuple::new((9.0, 10.0, 11.0, 12.0)),
            Tuple::new((13.5, 14.5, 15.5, 16.5)),
        ));

        assert_eq!(matrix[3][3], 16.5);

        matrix[0][1] = 0.0;

        assert_eq!(matrix[0][1], 0.0);
    }
}
