use crate::{point::Point, tuple::Tuple};
use float_cmp::approx_eq;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub(crate) enum IntoVectorError {
    // W not 0.0
    BadW,
    WrongLength,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z, w: 0.0 }
    }

    pub fn zero() -> Self {
        Vector::new(0.0, 0.0, 0.0)
    }

    pub fn x_unit() -> Self {
        Vector::new(1.0, 0.0, 0.0)
    }

    pub fn y_unit() -> Self {
        Vector::new(0.0, 1.0, 0.0)
    }

    pub fn z_unit() -> Self {
        Vector::new(0.0, 0.0, 1.0)
    }

    pub fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        self / magnitude
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2}, 0.00)", self.x, self.y, self.z)
    }
}

impl TryFrom<Tuple> for Vector {
    type Error = IntoVectorError;
    fn try_from(t: Tuple) -> Result<Self, Self::Error> {
        if t.size() != 4 {
            return Err(IntoVectorError::WrongLength);
        }
        if t[3] == 0.0 {
            return Ok(Vector::new(t[0], t[1], t[2]));
        }
        Err(IntoVectorError::BadW)
    }
}

impl PartialEq for Vector {
    fn eq(&self, rhs: &Vector) -> bool {
        approx_eq!(f32, self.x, rhs.x)
            && approx_eq!(f32, self.y, rhs.y)
            && approx_eq!(f32, self.z, rhs.z)
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Sub<Point> for Vector {
    type Output = Tuple;

    fn sub(self, rhs: Point) -> Self::Output {
        Tuple::new((self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, -1.0))
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::zero() - self
    }
}

impl std::ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::BitXor<Vector> for Vector {
    type Output = f32;

    fn bitxor(self, rhs: Vector) -> Self::Output {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl std::ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{point::Point, tuple::Tuple, vector::Vector};

    #[test]
    fn vector_has_w_eq_0() {
        let vector = Vector::new(0.0, 2.0, 3.0);
        assert_eq!(vector.x, 0.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);
        assert_eq!(vector.w, 0.0);
    }
    #[test]
    fn try_vector_from_tuple() {
        let ok_vector = Vector::try_from(Tuple::new((0.0, -1.0, 2.0, 0.0)));
        let err_vector = Vector::try_from(Tuple::new((0.0, -1.0, 2.0, 1.0)));
        assert!(ok_vector.is_ok());
        if let Ok(vector) = ok_vector {
            assert_eq!(vector, Vector::new(0.0, -1.0, 2.0));
        }
        assert!(err_vector.is_err());
    }
    #[test]
    fn two_vectors_approx_eq() {
        let vector1 = Vector::new(0.0 - 0.5 + 0.3, 2.2, 3.0 / 1.5);
        let vector2 = Vector::new(-0.2, 1.1 * 2.0, 2.0);
        assert_eq!(vector1, vector2);
    }
    #[test]
    fn two_vectors_approx_ne() {
        let vector1 = Vector::new(0.000001, 2.0, 3.0);
        let vector2 = Vector::new(0.0, 2.0, 3.0);
        assert_ne!(vector1, vector2);
    }
    #[test]
    fn add_two_vectors() {
        let vector1 = Vector::new(-4.0, 2.0, -3.0);
        let vector2 = Vector::new(0.0, 2.0, 3.0);
        let new_vector = vector1 + vector2;
        assert_eq!(new_vector, Vector::new(-4.0, 4.0, 0.0));
        assert_eq!(new_vector.w, 0.0);
    }
    #[test]
    fn add_point_to_vector() {
        let vector = Vector::new(-4.0, 2.0, -3.0);
        let point = Point::new(0.0, 2.0, 3.0);
        let new_point = vector + point;
        assert_eq!(new_point, Point::new(-4.0, 4.0, 0.0));
        assert_eq!(new_point.w, 1.0);
    }
    #[test]
    fn sub_two_vectors() {
        let vector1 = Vector::new(-4.0, 2.0, -3.0);
        let vector2 = Vector::new(0.0, 2.0, 3.0);
        let new_vector = vector1 - vector2;
        assert_eq!(new_vector, Vector::new(-4.0, 0.0, -6.0));
        assert_eq!(new_vector.w, 0.0);
    }
    #[test]
    fn sub_point_from_vector() {
        let vector = Vector::new(-4.0, 2.0, -3.0);
        let point = Point::new(0.0, 2.0, 3.0);
        let new_tuple = vector - point;
        assert_eq!(new_tuple, Tuple::new((-4.0, 0.0, -6.0, -1.0)));
    }
    #[test]
    fn neg_vector() {
        let vector = Vector::new(-4.0, 2.0, 0.0);
        let new_vector = -vector;
        assert_eq!(new_vector, Vector::new(4.0, -2.0, 0.0));
        assert_eq!(new_vector.w, 0.0);
    }
    #[test]
    fn scale_vector() {
        let vector = Vector::new(-4.0, 2.0, 0.0);
        let new_vector = vector * 1.5;
        assert_eq!(new_vector, Vector::new(-6.0, 3.0, 0.0));
    }
    #[test]
    fn scale_vector_by_division() {
        let vector = Vector::new(-4.0, 2.0, 0.0);
        let new_vector = vector / 2.0;
        assert_eq!(new_vector, Vector::new(-2.0, 1.0, 0.0));
    }
    #[test]
    fn vector_magnitude() {
        let vector = Vector::new(3.0, 4.0, 12.0);
        assert_eq!(vector.magnitude(), 13.0);
    }
    #[test]
    fn vector_normalization() {
        let vector = Vector::new(3.0, -4.0, 0.0);
        assert_eq!(vector.normalize(), Vector::new(0.6, -0.8, 0.0));
    }
    #[test]
    fn vector_dot_product() {
        let dot_product = Vector::new(1.0, 2.0, 3.0) ^ Vector::new(2.0, 3.0, 4.0);
        assert_eq!(dot_product, 20.0);
    }
    #[test]
    fn vector_cross_product() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(2.0, 3.0, 4.0);
        let cross_product1 = vector1 * vector2;
        let cross_product2 = vector2 * vector1;
        assert_eq!(cross_product1, Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(cross_product2, Vector::new(1.0, -2.0, 1.0));
    }
}
