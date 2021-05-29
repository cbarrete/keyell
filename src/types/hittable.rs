use std::f64::{EPSILON, INFINITY};

use crate::math::dot;
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

pub struct Plane<'a> {
    pub point: Point,
    pub normal: Normal,
    pub material: &'a dyn Material,
}

impl<'a> Hittable for Plane<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let normal = self.normal.outward().get().clone();
        let denom = dot(&normal, &ray.direction);
        if denom.abs() <= EPSILON {
            return None;
        }
        let diff = &self.point - &ray.origin;
        let travel = dot(&diff, &normal) / denom;
        if travel < t_min || travel > t_max {
            return None;
        }
        Some(Hit {
            travel,
            point: ray.at(travel),
            normal: self.normal.clone(),
            material: self.material,
        })
    }
}
