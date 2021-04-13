use crate::types::{Material, Ray, Vec3};

pub struct Hit<'a> {
    pub travel: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

impl<H: Hittable> Hittable for Vec<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        // TODO could probable be refactored as a fold
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

impl Hittable for Box<dyn Hittable> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.as_ref().hit(ray, t_min, t_max)
    }
}
