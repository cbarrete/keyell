use std::f64::INFINITY;
use std::fs::File;
use rand::{thread_rng, Rng};
use std::rc::Rc;

mod math;
mod types;
mod ppm_writer;
mod physics;

use crate::ppm_writer::PPMWriter;
use crate::types::{Camera, Canvas, Color, Hit, Hittable, Ray, Sphere, Vec3, Diffuse, Material, Metal};

fn make_spheres() -> Vec<Sphere> {
    let mut spheres = Vec::with_capacity(4);
    let pale_diffuse: Rc<dyn Material> = Rc::new(Diffuse { color: Color::new(1., 0.9, 1.) });
    let red_diffuse = Rc::new(Diffuse { color: Color::new(0.9, 0.2, 0.3) });
    let blue_diffuse = Rc::new(Diffuse { color: Color::new(0.3, 0.2, 0.9) });
    let green_diffuse = Rc::new(Diffuse { color: Color::new(0.4, 0.8, 0.4) });
    let metal = Rc::new(Metal { color: Color::new(0.8, 0.8, 0.8), fuzz: 0. });
    let light_metal = Rc::new(Metal { color: Color::new(1., 1., 1.), fuzz: 0. });
    let fuzzed_metal = Rc::new(Metal { color: Color::new(1., 1., 1.), fuzz: 0.2 });
    // TODO should have Point instead of Vec3
    spheres.push(Sphere { center: Vec3::new(0., -100.1, -1.), radius: 100., material: green_diffuse });
    spheres.push(Sphere { center: Vec3::new(0.,    0.,  -1.), radius: 0.7,  material: pale_diffuse });
    spheres.push(Sphere { center: Vec3::new(0.2,   0., -0.2), radius: 0.1,  material: red_diffuse });
    spheres.push(Sphere { center: Vec3::new(-0.3,  0., -0.5), radius: 0.3,  material: metal });
    spheres.push(Sphere { center: Vec3::new(0.05, -0.05, -0.2), radius: 0.05, material: light_metal });
    spheres.push(Sphere { center: Vec3::new(0.1,  0.1, -0.3), radius: 0.1,  material: fuzzed_metal });
    spheres.push(Sphere { center: Vec3::new(0.,    0.,   1.), radius: 0.5,  material: blue_diffuse });
    spheres
}

// N is a surface normal (a unit vector pointing from center to surface)
// we just give each a nice unique color so that we can discern the surface
fn color_surface_normal(normal: &Vec3) -> Color {
    0.5 * Color::new(normal.x + 1., normal.y + 1., normal.z + 1.)
}

// TODO stronger type for Scene
fn color_hit(scene: &dyn Hittable, ray: &Ray, hit: &Hit, remaining_diffusions: usize) -> Color {
    if remaining_diffusions <= 0 {
        return Color::black();
    }
    match hit.material.scatter(ray, hit) {
        Some((scattered, attenuation)) => attenuation * ray_color(&scattered, scene, remaining_diffusions - 1),
        None => Color::black(),
    }
}

// blue to white grandient based on y
fn color_gradient_background(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.unit().y + 1.);
    t * Color::new(0.5, 0.7, 1.0) + (1. - t) * Color::new(1., 1., 1.)
}

fn ray_color(ray: &Ray, scene: &dyn Hittable, remaining_diffusions: usize) -> Color {
    match scene.hit(ray, 0.001, INFINITY) {
        Some(hit) => color_hit(scene, ray, &hit, remaining_diffusions),
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
