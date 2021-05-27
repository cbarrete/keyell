use crate::math::same_orientation;
use crate::physics::{reflect, refract};
use crate::types::{Color, Hit, Ray};

use super::UnitVec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}

pub struct Diffuse {
    pub color: Color,
}

impl Material for Diffuse {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let scatter_direction = hit.normal.outward().get() + UnitVec3::random().get();
        let attenuation = self.color.clone();
        let scattered = Ray {
            origin: hit.point.clone(),
            direction: scatter_direction,
        };
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    pub color: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let reflected = reflect(&ray.direction.unit(), &hit.normal.outward());
        let scattered = Ray {
            origin: hit.point.clone(),
            direction: reflected + self.fuzz * UnitVec3::random().get(),
        };
        let attenuation = self.color.clone();

        if same_orientation(&scattered.direction, &hit.normal.outward().get()) {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let refraction_ratio = match hit.normal {
            super::Normal::Inward(_) => 1. / self.refraction_index,
            super::Normal::Outward(_) => self.refraction_index,
        };
        let refracted = refract(
            &ray.direction.unit(),
            &hit.normal.outward(),
            refraction_ratio,
        );
        let scattered = Ray {
            origin: hit.point.clone(),
            direction: refracted,
        };
        Some((scattered, Color::WHITE))
    }
}
