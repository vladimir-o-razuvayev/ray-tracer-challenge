use float_cmp::approx_eq;
use std::fmt;

pub(crate) const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

pub(crate) const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};

#[derive(Clone, Debug)]
pub(crate) struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = (self.r.clamp(0.0, 1.0).round() * 255.0) as u8;
        let g = (self.g.clamp(0.0, 1.0).round() * 255.0) as u8;
        let b = (self.b.clamp(0.0, 1.0).round() * 255.0) as u8;
        write!(f, "{} {} {}\n", r, g, b)
    }
}

impl PartialEq for Color {
    fn eq(&self, rhs: &Color) -> bool {
        approx_eq!(f32, self.r, rhs.r)
            && approx_eq!(f32, self.g, rhs.g)
            && approx_eq!(f32, self.b, rhs.b)
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl std::ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl std::ops::Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
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
    #[test]
    fn add_two_colors() {
        let color1 = Color::new(0.5, 1.0, 0.3);
        let color2 = Color::new(-0.4, 1.0, -0.3);
        let new_color = color1 + color2;
        assert_eq!(new_color, Color::new(0.1, 2.0, 0.0));
    }
    #[test]
    fn sub_two_colors() {
        let color1 = Color::new(0.5, 1.0, 0.3);
        let color2 = Color::new(-0.4, 1.0, -0.3);
        let new_color = color1 - color2;
        assert_eq!(new_color, Color::new(0.9, 0.0, 0.6));
    }
    #[test]
    fn scale_color() {
        let color = Color::new(-4.0, 2.0, 0.0);
        let new_color = color * 1.5;
        assert_eq!(new_color, Color::new(-6.0, 3.0, 0.0));
    }
    #[test]
    fn scale_color_by_division() {
        let color = Color::new(-4.0, 2.0, 0.0);
        let new_color = color / 2.0;
        assert_eq!(new_color, Color::new(-2.0, 1.0, 0.0));
    }
    #[test]
    fn color_hadamard_product() {
        let hadamard_product = Color::new(0.5, 1.0, 0.0) * Color::new(0.25, 0.8, 3.0);
        assert_eq!(hadamard_product, Color::new(0.125, 0.8, 0.0));
    }
}
