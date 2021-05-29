use std::f64::INFINITY;
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
        let mut closest_hit = None;
        let mut closest_travel = t_max;
        for hittable in self {
            if let Some(hit) = hittable.hit(ray, t_min, closest_travel) {
                closest_travel = hit.travel;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}

pub struct Background<'a> {
    pub material: &'a dyn Material,
}

impl<'a> Hittable for Background<'a> {
    fn hit(&self, ray: &Ray, _t_min: f64, t_max: f64) -> Option<Hit> {
        if t_max == INFINITY {
            Some(Hit {
                travel: t_max,
                point: ray.at(t_max),
                material: self.material,
                normal: Normal::Inward((-&ray.direction).unit()),
            })
        } else {
            None
        }
    }
}
