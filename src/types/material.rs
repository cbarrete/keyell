use crate::types::{Vec3, Ray, Hit, Color};
use crate::physics::reflect;
use crate::math::same_sense;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}

pub struct Diffuse {
    pub color: Color,
}

impl Material for Diffuse {
    fn scatter(&self, _in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let scatter_direction = &hit.normal + &Vec3::random_unit_vector();
        let attenuation = self.color.clone();
        let scattered = Ray { origin: hit.point.clone(), direction: scatter_direction };
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    pub color: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, in_ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let scattered = reflect(&in_ray.direction.unit(), &hit.normal);
        let scattered = Ray { origin: hit.point.clone(), direction: scattered + self.fuzz * &Vec3::random_unit_vector() };
        let attenuation = self.color.clone();
        if same_sense(&scattered.direction, &hit.normal) {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
