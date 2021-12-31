use crate::math::deg_to_radians;
use crate::render::Ray;
use crate::types::{Point, Vec3};

pub struct Degrees(f32);

impl Degrees {
    pub fn new(d: f32) -> Self {
        Self(d)
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct Camera {
    to_lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    position: Point,
}

impl Camera {
    pub fn from_canvas(canvas: &Canvas, position: Point, fov: Degrees) -> Camera {
        let aspect_ratio = canvas.width as f32 / canvas.height as f32;
        let focal_length = 1.;
        let h = 2. * (deg_to_radians(fov.0) / 2.).tan() * focal_length;
        let v = h / aspect_ratio;
        Camera {
            to_lower_left_corner: Vec3::new(-h / 2., focal_length, -v / 2.),
            horizontal: Vec3::new(h, 0., 0.),
            vertical: Vec3::new(0., 0., v),
            position,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.position.clone(),
            direction: &self.to_lower_left_corner + u * &self.horizontal + v * &self.vertical,
        }
    }
}
