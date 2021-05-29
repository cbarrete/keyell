use std::f64::consts::PI;

use super::Color;
use super::Hit;
use super::Normal;

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
            Normal::Inward(v) => v,
            Normal::Outward(v) => v,
        };
        let f = |coord: f64| (PI * coord).sin() + 1.;
        0.5 * Color::new(f(n.get().x), f(n.get().y), f(n.get().z))
    }
}

pub struct ZGradient {
    pub bottom: Color,
    pub top: Color,
}

impl Colorer for ZGradient {
    fn color(&self, hit: &Hit) -> Color {
        let t = 0.5 * (hit.normal.outward().get().z + 1.);
        t * &self.top + (1. - t) * &self.bottom
    }
}
