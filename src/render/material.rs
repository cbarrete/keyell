use rand::rngs::SmallRng;

use crate::math::{dot, same_orientation};
use crate::physics::{reflect, refract};
use crate::render::{Color, Colorer, Hit, Ray};
use crate::types::{Normal, UnitVec3};

pub enum Interaction {
    Source(Source),
    Bounce(Bounce),
    Nothing,
}

impl Interaction {
    pub fn bounce(scattered: Ray, attenuation: Color) -> Self {
        Self::Bounce(Bounce {
            scattered,
            attenuation,
        })
    }

    pub fn source(color: Color) -> Self {
        Self::Source(Source { color })
    }
}

pub struct Bounce {
    pub scattered: Ray,
    pub attenuation: Color,
}

pub struct Source {
    pub color: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut SmallRng) -> Interaction;
}

pub struct Diffuse {
    pub colorer: Colorer,
}

impl Material for Diffuse {
    fn scatter(&self, _ray: &Ray, hit: &Hit, rng: &mut SmallRng) -> Interaction {
        let scatter_direction = hit.normal.outward().get() + UnitVec3::random(rng).get();
        let scattered = Ray {
            origin: hit.point.clone(),
            direction: scatter_direction,
        };
        Interaction::bounce(scattered, self.colorer.color(hit))
    }
}

pub struct Metal {
    pub colorer: Colorer,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut SmallRng) -> Interaction {
        let reflected = reflect(&ray.direction.unit(), &hit.normal.outward());
        let scattered = Ray {
            origin: hit.point.clone(),
            direction: reflected + self.fuzz * UnitVec3::random(rng).get(),
        };
        if same_orientation(&scattered.direction, hit.normal.outward().get()) {
            Interaction::bounce(scattered, self.colorer.color(hit))
        } else {
            Interaction::Nothing
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f32,
    pub colorer: Colorer,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit, _: &mut SmallRng) -> Interaction {
        let refraction_ratio = match hit.normal {
            Normal::Inward(_) => 1. / self.refraction_index,
            Normal::Outward(_) => self.refraction_index,
        };
        let unit_direction = ray.direction.unit();
        let outward_normal = hit.normal.outward();

        let cos_theta = dot(&-unit_direction.get(), outward_normal.get());
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
        Interaction::bounce(scattered, self.colorer.color(hit))
    }
}

pub struct Light {
    pub colorer: Colorer,
}

impl Material for Light {
    fn scatter(&self, _ray: &Ray, hit: &Hit, _: &mut SmallRng) -> Interaction {
        Interaction::source(self.colorer.color(hit))
    }
}
