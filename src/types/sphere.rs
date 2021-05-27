use crate::math::dot;
use crate::types::{Hit, Hittable, Material, Normal, Point, Ray};

use super::UnitVec3;

pub struct Sphere<'a> {
    pub center: Point,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = &ray.origin - &self.center;
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let disc = half_b * half_b - a * c;

        // no solution
        if disc <= 0. {
            return None;
        }

        // computes useful useful values about the hit
        let compute_hit = |travel: f64| {
            let point = ray.at(travel);
            let normal_vec = (&point - &self.center) / self.radius;
            let normal = if dot(&ray.direction, &normal_vec) > 0. {
                Normal::Inward(UnitVec3::unchecked_from(&normal_vec))
            } else {
                Normal::Outward(UnitVec3::unchecked_from(&normal_vec))
            };
            Some(Hit {
                travel,
                normal,
                point,
                material: self.material,
            })
        };

        // try first solution
        let t = (-half_b - disc.sqrt()) / a;
        if t < t_max && t > t_min {
            return compute_hit(t);
        }

        // try second solution
        let t = (-half_b + disc.sqrt()) / a;
        if t < t_max && t > t_min {
            return compute_hit(t);
        }

        None
    }
}
