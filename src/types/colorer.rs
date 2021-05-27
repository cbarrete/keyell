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

pub struct Bubblegum {}

impl Colorer for Bubblegum {
    fn color(&self, hit: &Hit) -> Color {
        let n = match &hit.normal {
            super::Normal::Inward(v) => v,
            super::Normal::Outward(v) => v,
        };
        0.5 * Color::new(n.get().x + 1., n.get().y + 1., n.get().z + 1.)
    }
}
