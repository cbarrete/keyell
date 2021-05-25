use crate::types::{Material, Normal, Point, Ray};

pub struct Hit<'a> {
    pub travel: f64,
    pub point: Point,
    pub normal: Normal,
    pub material: &'a dyn Material,
}

pub trait Hittable {
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
