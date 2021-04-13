use crate::types::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray { origin: origin.clone(), direction: direction.clone() }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        &self.origin + &(t * &self.direction)
    }
}
