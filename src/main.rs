use std::f64::INFINITY;
use std::fs::File;

mod math;
mod types;
mod ppm_writer;

use crate::ppm_writer::PPMWriter;
use crate::types::{Hittable, Ray, Sphere, Vec3};

fn make_spheres() -> Vec<Sphere> {
    let mut spheres = Vec::with_capacity(4);
    spheres.push(Sphere { center: Vec3::new(0.,   0., -1.),  radius: 0.4 });
    spheres.push(Sphere { center: Vec3::new(1.,  0.2, -1.), radius: 0.3 });
    spheres.push(Sphere { center: Vec3::new(-0.5, 0., -1.3),  radius: 0.6 });
    spheres
}

// N is a surface normal (a unit vector pointing from center to surface)
// we just give each a nice unique color so that we can discern the surface
fn color_surface_normal(N: &Vec3) -> Vec3 {
    0.5 * Vec3::new(N.x + 1., N.y + 1., N.z + 1.)
}

// blue to white grandient based on y
fn color_gradient_background(ray: &Ray) -> Vec3 {
    let t = 0.5 * (ray.direction.unit().y + 1.);
    t * Vec3::new(0.5, 0.7, 1.0) + (1. - t) * Vec3::new(1., 1., 1.)
}

fn ray_color(ray: &Ray, scene: &dyn Hittable) -> Vec3 {
    match scene.hit(ray, 0., INFINITY) {
        Some(hit) => color_surface_normal(&hit.normal.unit()),
        None => color_gradient_background(ray),
    }
}

fn main() -> Result<(), std::io::Error> {
    let width  = 500;
    let height = 300;

    let mut writer = PPMWriter::new(File::create("out.ppm")?, width, height);
    writer.write_header()?;

    let lower_left_corner = Vec3::new(-2.5, -1.5, -1.);
    let horizontal = Vec3::new(5., 0., 0.);
    let vertical = Vec3::new(0., 3., 0.);
    let camera = Vec3::new(0., 0., 0.);

    let scene = make_spheres();

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f64 / width as f64;
            let v = j as f64 / height as f64;
            writer.write(&ray_color(
                    &Ray::new(
                        &camera,
                        &(lower_left_corner.clone() + u * &horizontal + v * &vertical)),
                    &scene)
                )?;
        }
    }

    Ok(())
}
