use rand::rngs::SmallRng;

use crate::math::{dot, same_orientation};
use crate::physics::{reflect, refract};
use crate::render::{Color, Colorer, Hit, Ray};
use crate::types::{Normal, UnitVec3};

use serde::{Deserialize, Serialize};

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

#[derive(Clone, Serialize, Deserialize)]
pub enum Material {
    Diffuse(Colorer),
    Metal {
        colorer: Colorer,
        fuzz: f32,
    },
    Dielectric {
        refraction_index: f32,
        colorer: Colorer,
    },
    Light(Colorer),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut SmallRng) -> Interaction {
        match self {
            Material::Diffuse(colorer) => {
                let scatter_direction = hit.normal.outward().get() + UnitVec3::random(rng).get();
                let scattered = Ray {
                    origin: hit.point.clone(),
                    direction: scatter_direction,
                };
                Interaction::bounce(scattered, colorer.color(hit))
            }
            Material::Metal { colorer, fuzz } => {
                let reflected = reflect(&ray.direction.unit(), &hit.normal.outward());
                let scattered = Ray {
                    origin: hit.point.clone(),
                    direction: reflected + *fuzz * UnitVec3::random(rng).get(),
                };
                if same_orientation(&scattered.direction, hit.normal.outward().get()) {
                    Interaction::bounce(scattered, colorer.color(hit))
                } else {
                    Interaction::Nothing
                }
            }
            Material::Dielectric {
                refraction_index,
                colorer,
            } => {
                let refraction_ratio = match hit.normal {
                    Normal::Inward(_) => 1. / refraction_index,
                    Normal::Outward(_) => *refraction_index,
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
                Interaction::bounce(scattered, colorer.color(hit))
            }
            Material::Light(colorer) => Interaction::source(colorer.color(hit)),
        }
    }

    pub fn get_colorer(&self) -> Colorer {
        match self {
            Material::Diffuse(colorer)
            | Material::Metal { colorer, .. }
            | Material::Dielectric { colorer, .. }
            | Material::Light(colorer) => colorer.clone(),
        }
    }
}
