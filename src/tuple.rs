use float_cmp::approx_eq;

use crate::point::Point;
use crate::vector::Vector;

use std::boxed::Box;

#[derive(Clone, Debug)]
pub(crate) struct Tuple {
    data: Vec<f32>,
}

#[macro_export]
macro_rules! tup {
    () => (
        Tuple::new(Vec::new())
    );
    ($($x:expr),+ $(,)?) => (
        Tuple::new(<[_]>::into_vec(
            Box::new([$($x),+])
        ))
    );
}

impl Tuple {
    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn w(&self) -> f32 {
        self.data[3]
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn new(data: Vec<f32>) -> Self {
        Tuple { data }
    }

    pub fn zero(length: usize) -> Self {
        Tuple::new(vec![0.0; length])
    }

    pub fn magnitude(&self) -> f32 {
        self.data
            .iter()
            .fold(0.0, |acc, value| acc + (value * value))
            .sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        self / magnitude
    }
}

impl PartialEq for Tuple {
    fn eq(&self, t: &Tuple) -> bool {
        self.data
            .iter()
            .zip(t.data.iter())
            .fold(true, |acc, (v1, v2)| acc && approx_eq!(f32, *v1, *v2))
    }
}

impl From<Vector> for Tuple {
    fn from(v: Vector) -> Self {
        tup![v.x, v.y, v.z, 0.0]
    }
}

impl From<Point> for Tuple {
    fn from(p: Point) -> Self {
        tup![p.x, p.y, p.z, 1.0]
    }
}

impl std::ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(v1, v2)| v1 + v2)
                .collect(),
        )
    }
}

impl std::ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.data
                .iter()
                .zip(rhs.data.iter())
                .map(|(v1, v2)| v1 - v2)
                .collect(),
        )
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::zero(self.data.len()) - self
    }
}

impl std::ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple::new(self.data.iter().map(|value| value * rhs).collect())
    }
}

impl std::ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple::new(self.data.iter().map(|value| value / rhs).collect())
    }
}

impl std::ops::Index<usize> for Tuple {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for Tuple {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{point::Point, tuple::Tuple, vector::Vector};

    #[test]
    fn two_tuples_approx_eq() {
        let tuple1 = tup![0.0 - 0.5 + 0.3, 2.2, 3.0 / 1.5, -4.0];
        let tuple2 = tup![-0.2, 1.1 * 2.0, 2.0, 4.0 * -1.0];
        assert_eq!(tuple1, tuple2);
    }
    #[test]
    fn tuple_from_point() {
        let tuple = Tuple::from(Point::new(-4.0, 2.0, -3.0));
        assert_eq!(tuple, tup![-4.0, 2.0, -3.0, 1.0]);
    }
    #[test]
    fn tuple_from_vector() {
        let tuple = Tuple::from(Vector::new(-4.0, 2.0, -3.0));
        assert_eq!(tuple, tup![-4.0, 2.0, -3.0, 0.0]);
    }
    #[test]
    fn two_tuples_approx_ne() {
        let tuple1 = tup![0.000001, 2.0, 3.0, -4.0];
        let tuple2 = tup![0.0, 2.0, 3.0, -4.0];
        assert_ne!(tuple1, tuple2);
    }
    #[test]
    fn add_two_tuples() {
        let tuple1 = tup![-4.0, 2.0, -3.0, -4.0];
        let tuple2 = tup![0.0, 2.0, 3.0, -1.0];
        let new_tuple = tuple1 + tuple2;
        assert_eq!(new_tuple, tup![-4.0, 4.0, 0.0, -5.0]);
    }
    #[test]
    fn sub_two_tuples() {
        let tuple1 = tup![-4.0, 2.0, -3.0, -4.0];
        let tuple2 = tup![0.0, 2.0, 3.0, -1.0];
        let new_tuple = tuple1 - tuple2;
        assert_eq!(new_tuple, tup![-4.0, 0.0, -6.0, -3.0]);
    }
    #[test]
    fn neg_tuple() {
        let tuple = tup![-4.0, 2.0, 0.0, -3.0];
        let new_tuple = -tuple;
        assert_eq!(new_tuple, tup![4.0, -2.0, 0.0, 3.0]);
    }
    #[test]
    fn scale_tuple() {
        let tuple = tup![-4.0, 2.0, 0.0, -3.0];
        let new_tuple = tuple * 1.5;
        assert_eq!(new_tuple, tup![-6.0, 3.0, 0.0, -4.5]);
    }
    #[test]
    fn scale_tuple_by_division() {
        let tuple = tup![-4.0, 2.0, 0.0, -3.0];
        let new_tuple = tuple / 2.0;
        assert_eq!(new_tuple, tup![-2.0, 1.0, 0.0, -1.5]);
    }
    #[test]
    fn access_tuple_by_index() {
        let mut tuple = tup![-4.0, 2.0, 0.0, -3.0];
        assert_eq!(tuple[3], -3.0);
        tuple[2] = 1.0;
        assert_eq!(tuple[2], 1.0)
    }
}
