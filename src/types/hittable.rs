use crate::types::{Ray, Vec3};

pub struct Hit {
    pub travel: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hittable where {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
