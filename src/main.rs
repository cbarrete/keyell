use std::f64::INFINITY;
use std::fs::File;
use rand::{thread_rng, Rng};

mod math;
mod types;
mod ppm_writer;

use crate::ppm_writer::PPMWriter;
use crate::types::{Camera, Canvas, Color, Hittable, Ray, Sphere, Vec3};

fn make_spheres() -> Vec<Sphere> {
    let mut spheres = Vec::with_capacity(4);
    spheres.push(Sphere { center: Vec3::new(0.,   0., -1.),  radius: 0.4 });
    spheres.push(Sphere { center: Vec3::new(1.,  0.2, -1.), radius: 0.3 });
    spheres.push(Sphere { center: Vec3::new(-0.5, 0., -1.3),  radius: 0.6 });
    spheres
}

// N is a surface normal (a unit vector pointing from center to surface)
// we just give each a nice unique color so that we can discern the surface
fn color_surface_normal(N: &Vec3) -> Color {
    0.5 * Color::new(N.x + 1., N.y + 1., N.z + 1.)
}

// blue to white grandient based on y
fn color_gradient_background(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.unit().y + 1.);
    t * Color::new(0.5, 0.7, 1.0) + (1. - t) * Color::new(1., 1., 1.)
}

fn ray_color(ray: &Ray, scene: &dyn Hittable) -> Color {
    match scene.hit(ray, 0., INFINITY) {
        Some(hit) => color_surface_normal(&hit.normal.unit()),
        None => color_gradient_background(ray),
    }
}

fn main() -> Result<(), std::io::Error> {
    let canvas = Canvas { width: 500, height: 300 };
    let samples_per_pixel = 100;

    let mut writer = PPMWriter::new(File::create("out.ppm")?, &canvas);
    writer.write_header()?;

    let camera = Camera::from_canvas(&canvas);
    let scene = make_spheres();

    let mut rng = thread_rng();

    for j in (0..canvas.height).rev() {
        for i in 0..canvas.width {
            let mut color = Color::black();
            for _ in 0..samples_per_pixel {
                let u = (rng.gen_range(0., 1.) + i as f64) / canvas.width as f64;
                let v = (rng.gen_range(0., 1.) + j as f64) / canvas.height as f64;
                color = color + ray_color(&camera.get_ray(u, v), &scene);
            }
            writer.write(&(color / samples_per_pixel as f64))?;
        }
    }

    Ok(())
}
