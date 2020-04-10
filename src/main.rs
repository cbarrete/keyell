use std::f64::INFINITY;
use std::fs::File;
use rand::{thread_rng, Rng};

mod math;
mod types;
mod ppm_writer;

use crate::ppm_writer::PPMWriter;
use crate::types::{Camera, Canvas, Color, Hit, Hittable, Ray, Sphere, Vec3};

fn make_spheres() -> Vec<Sphere> {
    let mut spheres = Vec::with_capacity(4);
    spheres.push(Sphere { center: Vec3::new(0.,   0.,   -1.), radius: 0.7 });
    spheres.push(Sphere { center: Vec3::new(0., -100.1, -1.), radius: 100. });
    spheres
}

// N is a surface normal (a unit vector pointing from center to surface)
// we just give each a nice unique color so that we can discern the surface
fn color_surface_normal(normal: &Vec3) -> Color {
    0.5 * Color::new(normal.x + 1., normal.y + 1., normal.z + 1.)
}

// bounce randomly until some light is hit
fn color_diffused(scene: &dyn Hittable, hit: &Hit, max_diffusion: usize) -> Color {
    if max_diffusion == 0 {
        return Color::black()
    }
    let target = &hit.point + &hit.normal + Vec3::random_in_unit_sphere();
    let new_ray = Ray {
        origin: hit.point.clone(),
        direction: &target - &hit.point,
    };
    0.5 * ray_color(&new_ray, scene, max_diffusion - 1)
}

// blue to white grandient based on y
fn color_gradient_background(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.unit().y + 1.);
    t * Color::new(0.5, 0.7, 1.0) + (1. - t) * Color::new(1., 1., 1.)
}

fn ray_color(ray: &Ray, scene: &dyn Hittable, max_diffusion: usize) -> Color {
    match scene.hit(ray, 0., INFINITY) {
        Some(hit) => color_diffused(scene, &hit, max_diffusion),
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
                color = color + ray_color(&camera.get_ray(u, v), &scene, 50);
            }
            writer.write(&(color / samples_per_pixel as f64))?;
        }
    }

    Ok(())
}
