use std::f32::consts::PI;

use crate::render::Color;
use crate::render::Hit;
use crate::types::Normal;

pub enum Colorer {
    ZGradient { bottom: Color, top: Color },
    Solid(Color),
    Bubblegum,
}

impl Colorer {
    pub fn color(&self, hit: &Hit) -> Color {
        match self {
            Colorer::ZGradient { bottom, top } => {
                let t = 0.5 * (hit.normal.outward().get().z + 1.);
                t * top + (1. - t) * bottom
            }
            Colorer::Solid(color) => color.clone(),
            Colorer::Bubblegum => {
                let n = match &hit.normal {
                    Normal::Inward(v) | Normal::Outward(v) => v,
                };
                let f = |coord: f32| (PI * coord).sin() + 1.;
                0.5 * Color::new(f(n.get().x), f(n.get().y), f(n.get().z))
            }
        }
    }
}
