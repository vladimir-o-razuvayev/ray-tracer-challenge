use crate::{tup, tuple::Tuple, vector::Vector};
use float_cmp::approx_eq;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub(crate) enum IntoPointError {
    // W not 1.0
    BadW,
    WrongLength,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z, w: 1.0 }
    }

    pub fn origin() -> Self {
        Point::new(0.0, 0.0, 0.0)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2}, 1.00)", self.x, self.y, self.z)
    }
}

impl TryFrom<Tuple> for Point {
    type Error = IntoPointError;
    fn try_from(t: Tuple) -> Result<Self, Self::Error> {
        if t.size() != 4 {
            return Err(IntoPointError::WrongLength);
        }
        if t[3] == 1.0 {
            return Ok(Point::new(t[0], t[1], t[2]));
        } else {
        }
        return Err(IntoPointError::BadW);
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Point) -> bool {
        approx_eq!(f32, self.x, rhs.x)
            && approx_eq!(f32, self.y, rhs.y)
            && approx_eq!(f32, self.z, rhs.z)
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Tuple;

    fn add(self, rhs: Point) -> Self::Output {
        tup![self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, 2.0]
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Neg for Point {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        tup![-self.x, -self.y, -self.z, -1.0]
    }
}

#[cfg(test)]
mod tests {
    use crate::{point::Point, tup, tuple::Tuple, vector::Vector};

    #[test]
    fn point_has_w_eq_1() {
        let point = Point::new(0.0, 2.0, 3.0);
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 3.0);
        assert_eq!(point.w, 1.0);
    }
    #[test]
    fn try_point_from_tuple() {
        let ok_point = Point::try_from(tup![0.0, -1.0, 2.0, 1.0]);
        let err_point = Point::try_from(tup![0.0, -1.0, 2.0, 0.0]);
        assert!(ok_point.is_ok());
        if let Ok(point) = ok_point {
            assert_eq!(point, Point::new(0.0, -1.0, 2.0));
        }
        assert!(err_point.is_err());
    }
    #[test]
    fn two_points_approx_eq() {
        let point1 = Point::new(0.0 - 0.5 + 0.3, 2.2, 3.0 / 1.5);
        let point2 = Point::new(-0.2, 1.1 * 2.0, 2.0);
        assert_eq!(point1, point2);
    }
    #[test]
    fn two_points_approx_ne() {
        let point1 = Point::new(0.000001, 2.0, 3.0);
        let point2 = Point::new(0.0, 2.0, 3.0);
        assert_ne!(point1, point2);
    }
    #[test]
    fn add_vector_to_point() {
        let point = Point::new(0.0, 2.0, 3.0);
        let vector = Vector::new(-4.0, 2.0, -3.0);
        let new_point = point + vector;
        assert_eq!(new_point, Point::new(-4.0, 4.0, 0.0));
    }
    #[test]
    fn add_two_points() {
        let point1 = Point::new(0.0, 2.0, 3.0);
        let point2 = Point::new(-4.0, 2.0, -3.0);
        let new_tuple = point1 + point2;
        assert_eq!(new_tuple, tup![-4.0, 4.0, 0.0, 2.0]);
    }
    #[test]
    fn sub_vector_from_point() {
        let point = Point::new(0.0, 2.0, 3.0);
        let vector = Vector::new(-4.0, 2.0, -3.0);
        let new_point = point - vector;
        assert_eq!(new_point, Point::new(4.0, 0.0, 6.0));
    }
    #[test]
    fn sub_two_points() {
        let point1 = Point::new(0.0, 2.0, 3.0);
        let point2 = Point::new(-4.0, 2.0, -3.0);
        let new_vector = point1 - point2;
        assert_eq!(new_vector, Vector::new(4.0, 0.0, 6.0));
    }
    #[test]
    fn neg_point() {
        let point = Point::new(-4.0, 2.0, 0.0);
        let new_tuple = -point;
        assert_eq!(new_tuple, tup![4.0, -2.0, 0.0, -1.0]);
    }
}
