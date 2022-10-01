use float_cmp::approx_eq;
use std::fmt;

#[derive(Debug)]
pub(crate) struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.r, self.g, self.b)
    }
}

impl PartialEq for Color {
    fn eq(&self, rhs: &Color) -> bool {
        approx_eq!(f32, self.r, rhs.r)
            && approx_eq!(f32, self.g, rhs.g)
            && approx_eq!(f32, self.b, rhs.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn two_colors_approx_eq() {
        let color1 = Color::new(0.0 + 0.5 - 0.3, 0.2, 1.0 / 1.5);
        let color2 = Color::new(0.2, 0.2, 0.66666667);
        assert_eq!(color1, color2);
    }
}
