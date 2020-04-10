use crate::types::{Hit, Hittable, Ray, Vec3};
use crate::math::dot;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = &ray.origin - &self.center;
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let disc = half_b * half_b - a * c;

        // no solution
        if disc <= 0. {
            return None
        }

        // computes useful useful values about the hit
        let compute_hit = |travel: f64| {
            let point = ray.at(travel);
            Some(Hit {
                travel,
                normal: &(&point - &self.center) / self.radius,
                point: point,
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
