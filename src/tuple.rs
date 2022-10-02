use float_cmp::approx_eq;

use crate::point::Point;
use crate::vector::Vector;

#[derive(Clone, Debug)]
pub(crate) struct Tuple {
    data: Vec<f32>,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple {
            data: vec![x, y, z, w],
        }
    }

    pub fn x(self) -> f32 {
        self.data[0]
    }

    pub fn y(self) -> f32 {
        self.data[1]
    }

    pub fn z(self) -> f32 {
        self.data[2]
    }

    pub fn w(self) -> f32 {
        self.data[3]
    }

    pub fn zero() -> Self {
        Tuple::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn magnitude(&self) -> f32 {
        ((self.x() * self.x())
            + (self.y() * self.y())
            + (self.z() * self.z())
            + (self.w() * self.w()))
        .sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        self / magnitude
    }
}

impl PartialEq for Tuple {
    fn eq(&self, t: &Tuple) -> bool {
        approx_eq!(f32, self.x(), t.x())
            && approx_eq!(f32, self.y(), t.y())
            && approx_eq!(f32, self.z(), t.z())
            && approx_eq!(f32, self.w(), t.w())
    }
}

impl From<Vector> for Tuple {
    fn from(v: Vector) -> Self {
        Tuple::new(v.x, v.y, v.z, 0.0)
    }
}

impl From<Point> for Tuple {
    fn from(p: Point) -> Self {
        Tuple::new(p.x, p.y, p.z, 1.0)
    }
}

impl std::ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
            self.w() + rhs.w(),
        )
    }
}

impl std::ops::Add<Vector> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Vector) -> Self::Output {
        Tuple::new(
            self.x() + rhs.x,
            self.y() + rhs.y,
            self.z() + rhs.z,
            self.w(),
        )
    }
}

impl std::ops::Add<Point> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Point) -> Self::Output {
        Tuple::new(
            self.x() + rhs.x,
            self.y() + rhs.y,
            self.z() + rhs.z,
            self.w() + 1.0,
        )
    }
}

impl std::ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
            self.w() - rhs.w(),
        )
    }
}

impl std::ops::Sub<Vector> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Vector) -> Self::Output {
        Tuple::new(
            self.x() - rhs.x,
            self.y() - rhs.y,
            self.z() - rhs.z,
            self.w(),
        )
    }
}

impl std::ops::Sub<Point> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Point) -> Self::Output {
        Tuple::new(
            self.x() - rhs.x,
            self.y() - rhs.y,
            self.z() - rhs.z,
            self.w() - 1.0,
        )
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::zero() - self
    }
}

impl std::ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
            self.w() * rhs,
        )
    }
}

impl std::ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple::new(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs,
            self.w() / rhs,
        )
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
        let tuple1 = Tuple::new(0.0 - 0.5 + 0.3, 2.2, 3.0 / 1.5, -4.0);
        let tuple2 = Tuple::new(-0.2, 1.1 * 2.0, 2.0, 4.0 * -1.0);
        assert_eq!(tuple1, tuple2);
    }
    #[test]
    fn tuple_from_point() {
        let tuple = Tuple::from(Point::new(-4.0, 2.0, -3.0));
        assert_eq!(tuple, Tuple::new(-4.0, 2.0, -3.0, 1.0));
    }
    #[test]
    fn tuple_from_vector() {
        let tuple = Tuple::from(Vector::new(-4.0, 2.0, -3.0));
        assert_eq!(tuple, Tuple::new(-4.0, 2.0, -3.0, 0.0));
    }
    #[test]
    fn two_tuples_approx_ne() {
        let tuple1 = Tuple::new(0.000001, 2.0, 3.0, -4.0);
        let tuple2 = Tuple::new(0.0, 2.0, 3.0, -4.0);
        assert_ne!(tuple1, tuple2);
    }
    #[test]
    fn add_two_tuples() {
        let tuple1 = Tuple::new(-4.0, 2.0, -3.0, -4.0);
        let tuple2 = Tuple::new(0.0, 2.0, 3.0, -1.0);
        let new_tuple = tuple1 + tuple2;
        assert_eq!(new_tuple, Tuple::new(-4.0, 4.0, 0.0, -5.0));
    }
    #[test]
    fn add_vector_to_tuple() {
        let tuple = Tuple::new(-4.0, 2.0, -3.0, -4.0);
        let vector = Vector::new(0.0, 2.0, 3.0);
        let new_tuple = tuple + vector;
        assert_eq!(new_tuple, Tuple::new(-4.0, 4.0, 0.0, -4.0));
    }
    #[test]
    fn add_point_to_tuple() {
        let tuple = Tuple::new(-4.0, 2.0, -3.0, -4.0);
        let point = Point::new(0.0, 2.0, 3.0);
        let new_tuple = tuple + point;
        assert_eq!(new_tuple, Tuple::new(-4.0, 4.0, 0.0, -3.0));
    }
    #[test]
    fn sub_two_tuples() {
        let tuple1 = Tuple::new(-4.0, 2.0, -3.0, -4.0);
        let tuple2 = Tuple::new(0.0, 2.0, 3.0, -1.0);
        let new_tuple = tuple1 - tuple2;
        assert_eq!(new_tuple, Tuple::new(-4.0, 0.0, -6.0, -3.0));
    }
    #[test]
    fn sub_vector_from_tuple() {
        let tuple = Tuple::new(-4.0, 2.0, -3.0, -4.0);
        let vector = Vector::new(0.0, 2.0, 3.0);
        let new_tuple = tuple - vector;
        assert_eq!(new_tuple, Tuple::new(-4.0, 0.0, -6.0, -4.0));
    }
    #[test]
    fn sub_point_from_tuple() {
        let tuple = Tuple::new(-4.0, 2.0, -3.0, -4.0);
        let point = Point::new(0.0, 2.0, 3.0);
        let new_tuple = tuple - point;
        assert_eq!(new_tuple, Tuple::new(-4.0, 0.0, -6.0, -5.0));
    }
    #[test]
    fn neg_tuple() {
        let tuple = Tuple::new(-4.0, 2.0, 0.0, -3.0);
        let new_tuple = -tuple;
        assert_eq!(new_tuple, Tuple::new(4.0, -2.0, 0.0, 3.0));
    }
    #[test]
    fn scale_tuple() {
        let tuple = Tuple::new(-4.0, 2.0, 0.0, -3.0);
        let new_tuple = tuple * 1.5;
        assert_eq!(new_tuple, Tuple::new(-6.0, 3.0, 0.0, -4.5));
    }
    #[test]
    fn scale_tuple_by_division() {
        let tuple = Tuple::new(-4.0, 2.0, 0.0, -3.0);
        let new_tuple = tuple / 2.0;
        assert_eq!(new_tuple, Tuple::new(-2.0, 1.0, 0.0, -1.5));
    }
    #[test]
    fn access_tuple_by_index() {
        let mut tuple = Tuple::new(-4.0, 2.0, 0.0, -3.0);
        assert_eq!(tuple[3], -3.0);
        tuple[2] = 1.0;
        assert_eq!(tuple[2], 1.0)
    }
}
