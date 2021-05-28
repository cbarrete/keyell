use crate::types::{Canvas, Point, Ray, Vec3};

#[derive(Debug)]
pub struct Camera {
    to_lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    eye: Point,
}

impl Camera {
    pub fn from_canvas(canvas: &Canvas) -> Camera {
        let aspect_ratio = canvas.width as f64 / canvas.height as f64;
        let focal_length = 1.;
        let v = 2.0;
        let h = aspect_ratio * v;
        Camera {
            to_lower_left_corner: Vec3::new(-h / 2., focal_length, -v / 2.),
            horizontal: Vec3::new(h, 0., 0.),
            vertical: Vec3::new(0., 0., v),
            eye: Point::new(0., 0., 0.),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.eye.clone(),
            direction: &self.to_lower_left_corner + u * &self.horizontal + v * &self.vertical,
        }
    }
}
