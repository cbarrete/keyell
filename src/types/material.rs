use crate::math::{dot, same_orientation};
use crate::physics::{reflect, refract};
use crate::types::{Color, Hit, Ray};

use super::Colorer;
use super::UnitVec3;

pub struct Bounce {
    pub scattered: Ray,
    pub attenuation: Color,
}

impl Bounce {
    fn new(scattered: Ray, attenuation: Color) -> Self {
        Self {
            scattered,
            attenuation,
        }
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Bounce>;
}

pub struct Diffuse<'a> {
    pub colorer: &'a dyn Colorer,
}

impl<'a> Material for Diffuse<'a> {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Bounce> {
        let scatter_direction = hit.normal.outward().get() + UnitVec3::random().get();
        let scattered = Ray {
            origin: hit.point.clone(),
            direction: scatter_direction,
        };
        Some(Bounce::new(scattered, self.colorer.color(hit)))
    }
}

pub struct Metal<'a> {
    pub colorer: &'a dyn Colorer,
    pub fuzz: f64,
}

impl<'a> Material for Metal<'a> {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Bounce> {
        let reflected = reflect(&ray.direction.unit(), &hit.normal.outward());
        let scattered = Ray {
            origin: hit.point.clone(),
            direction: reflected + self.fuzz * UnitVec3::random().get(),
        };
        if same_orientation(&scattered.direction, &hit.normal.outward().get()) {
            Some(Bounce::new(scattered, self.colorer.color(hit)))
        } else {
            None
        }
    }
}

pub struct Dielectric<'a> {
    pub refraction_index: f64,
    pub colorer: &'a dyn Colorer,
}

impl<'a> Material for Dielectric<'a> {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Bounce> {
        let refraction_ratio = match hit.normal {
            super::Normal::Inward(_) => 1. / self.refraction_index,
            super::Normal::Outward(_) => self.refraction_index,
        };
        let unit_direction = ray.direction.unit();
        let outward_normal = hit.normal.outward();

        let cos_theta = dot(&-unit_direction.get(), &outward_normal.get());
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();
        let can_refract = refraction_ratio * sin_theta <= 1.;

        let direction = if can_refract {
            refract(&unit_direction, &outward_normal, refraction_ratio)
        } else {
            reflect(&ray.direction.unit(), &hit.normal.outward())
        };

        let scattered = Ray {
            origin: hit.point.clone(),
            direction,
        };
        Some(Bounce::new(scattered, self.colorer.color(hit)))
    }
}
