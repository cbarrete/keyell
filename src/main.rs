use std::fs::File;

mod math;
mod types;
mod ppm_writer;

use crate::ppm_writer::PPMWriter;
use crate::types::{Ray, Vec3};
use crate::math::dot;

// if hit sphere: return distance of intersection from camera
// if not: None
fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = &ray.origin - center;
    let a = dot(&ray.direction, &ray.direction);
    let half_b = dot(&oc, &ray.direction);
    let c = dot(&oc, &oc) - radius * radius;
    let disc = half_b * half_b - a * c;
    if disc < 0. {
        None
    } else {
        Some((-half_b - disc.sqrt()) / a)
    }
}

// N is a surface normal (a unit vector pointing from center to surface)
// we just give each a nice unique color so that we can discern the surface
fn color_surface_normal(N: &Vec3) -> Vec3 {
    0.5 * Vec3::new(N.x + 1., N.y + 1., N.z + 1.)
}

fn ray_color(ray: &Ray) -> Vec3 {
    // sphere in the middle
    let center = Vec3::new(0., 0., -1.);
    match hit_sphere(&center, 0.5, ray) {
        Some(t) => color_surface_normal(&(ray.at(t) - center).unit()),
        None => {
            // background: blue to white gradient
            let t = 0.5 * (ray.direction.unit().y + 1.);
            t * Vec3::new(0.5, 0.7, 1.0) + (1. - t) * Vec3::new(1., 1., 1.)
        },
    }
}

fn main() -> Result<(), std::io::Error> {
    let width  = 200;
    let height = 100;

    let mut writer = PPMWriter::new(File::create("out.ppm")?, width, height);
    writer.write_header()?;

    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let camera = Vec3::new(0., 0., 0.);

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f64 / width as f64;
            let v = j as f64 / height as f64;
            writer.write(&ray_color(&Ray::new(
                        &camera,
                        &(lower_left_corner.clone() + u * &horizontal + v * &vertical))))?;
        }
    }

    Ok(())
}
