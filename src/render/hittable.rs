use std::f32::{EPSILON, INFINITY};

use crate::math::dot;
use crate::render::{Material, Ray};
use crate::types::{Normal, Point, UnitVec3};

use serde::{Deserialize, Serialize};

pub struct Hit<'a> {
    pub travel: f32,
    pub point: Point,
    pub normal: Normal,
    pub material: &'a Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = &ray.origin - &self.center;
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let disc = half_b.powi(2) - a * c;

        // no solution
        if disc <= 0. {
            return None;
        }

        // computes useful useful values about the hit
        let compute_hit = |travel: f32| {
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
                material: &self.material,
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

#[derive(Serialize, Deserialize)]
pub struct Background {
    pub material: Material,
}

impl Hittable for Background {
    fn hit(&self, ray: &Ray, _t_min: f32, t_max: f32) -> Option<Hit> {
        if t_max == INFINITY {
            Some(Hit {
                travel: t_max,
                point: ray.at(t_max),
                material: &self.material,
                normal: Normal::Inward((-&ray.direction).unit()),
            })
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Plane {
    pub point: Point,
    pub normal: Normal,
    pub material: Material,
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
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
            material: &self.material,
        })
    }
}
