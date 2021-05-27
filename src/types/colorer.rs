use super::Color;
use super::Hit;

pub trait Colorer {
    fn color(&self, hit: &Hit) -> Color;
}

pub struct Solid {
    color: Color,
}

impl Solid {
    pub const fn from_color(color: Color) -> Self {
        Self { color }
    }
}

impl Colorer for Solid {
    fn color(&self, _hit: &Hit) -> Color {
        self.color.clone()
    }
}
