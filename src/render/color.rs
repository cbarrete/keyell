#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const BLACK: Self = Self::new(0., 0., 0.);
    pub const WHITE: Self = Self::new(1., 1., 1.);

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub const fn grey(value: f64) -> Self {
        Self {
            r: value,
            g: value,
            b: value,
        }
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl std::ops::Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Color {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Color {
        (1. / rhs) * self
    }
}
