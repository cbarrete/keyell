use crate::math::gcd;
use crate::types::{Canvas, Point, Ray, Vec3};

#[derive(Debug)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Point,
}

impl Camera {
    pub fn from_canvas(canvas: &Canvas) -> Camera {
        let gcd = gcd(canvas.height, canvas.width);
        let h = (canvas.width / gcd) as f64;
        let v = (canvas.height / gcd) as f64;
        Camera {
            lower_left_corner: Vec3::new(-h / 2., -1., -v / 2.),
            horizontal: Vec3::new(h, 0., 0.),
            vertical: Vec3::new(0., 0., v),
            origin: Point::new(0., 0., 0.),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin.clone(),
            direction: self.lower_left_corner.clone() + u * &self.horizontal + v * &self.vertical,
        }
    }
}
