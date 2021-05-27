use super::UnitVec3;

pub enum Normal {
    Inward(UnitVec3),
    Outward(UnitVec3),
}

impl Normal {
    pub fn outward(&self) -> UnitVec3 {
        match self {
            Normal::Inward(v) => -v,
            Normal::Outward(v) => v.clone(),
        }
    }
}
