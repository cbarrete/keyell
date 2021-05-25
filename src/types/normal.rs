use crate::Vec3;

pub enum Normal {
    Inward(Vec3),
    Outward(Vec3),
}

impl Normal {
    pub fn outward(&self) -> Vec3 {
        match self {
            Normal::Inward(v) => -v,
            Normal::Outward(v) => v.clone(),
        }
    }
}
