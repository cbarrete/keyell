use crate::types::{Ray, Vec3};

pub struct Hit {
    pub travel: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hittable where {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

impl<H: Hittable> Hittable for Vec<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest = None;
        let mut closest_travel = t_max;
        for hittable in self {
            if let Some(hit) = hittable.hit(ray, t_min, closest_travel) {
                closest_travel = hit.travel;
                closest = Some(hit);
            }
        }
        closest
    }
}
