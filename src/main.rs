use rand::{thread_rng, Rng};
use std::fs::File;
use std::{f64::INFINITY, io::BufWriter};

mod math;
mod physics;
mod ppm_writer;
mod types;

use crate::ppm_writer::PpmWriter;
use crate::types::{
    Camera, Canvas, Color, Diffuse, Hit, Hittable, Metal, Point, Ray, Sphere, Vec3,
};

static PALE_DIFFUSE: Diffuse = Diffuse {
    color: Color::new(1., 0.9, 1.),
};
static RED_DIFFUSE: Diffuse = Diffuse {
    color: Color::new(0.9, 0.2, 0.3),
};
static BLUE_DIFFUSE: Diffuse = Diffuse {
    color: Color::new(0.3, 0.2, 0.9),
};
static GREEN_DIFFUSE: Diffuse = Diffuse {
    color: Color::new(0.4, 0.8, 0.4),
};
const METAL: Metal = Metal {
    color: Color::new(0.8, 0.8, 0.8),
    fuzz: 0.,
};
const LIGHT_METAL: Metal = Metal {
    color: Color::new(1., 1., 1.),
    fuzz: 0.,
};
const FUZZED_METAL: Metal = Metal {
    color: Color::new(1., 1., 1.),
    fuzz: 0.2,
};

fn make_spheres() -> Vec<Sphere<'static>> {
    let mut spheres = Vec::with_capacity(4);
    // TODO should have Point instead of Vec3
    spheres.push(Sphere {
        center: Point::new(0., -100.1, -1.),
        radius: 100.,
        material: &GREEN_DIFFUSE,
    });
    spheres.push(Sphere {
        center: Point::new(0., 0., -1.),
        radius: 0.7,
        material: &PALE_DIFFUSE,
    });
    spheres.push(Sphere {
        center: Point::new(0.2, 0., -0.2),
        radius: 0.1,
        material: &RED_DIFFUSE,
    });
    spheres.push(Sphere {
        center: Point::new(-0.3, 0., -0.5),
        radius: 0.3,
        material: &METAL,
    });
    spheres.push(Sphere {
        center: Point::new(0.05, -0.05, -0.2),
        radius: 0.05,
        material: &LIGHT_METAL,
    });
    spheres.push(Sphere {
        center: Point::new(0.1, 0.1, -0.3),
        radius: 0.1,
        material: &FUZZED_METAL,
    });
    spheres.push(Sphere {
        center: Point::new(0., 0., 1.),
        radius: 0.5,
        material: &BLUE_DIFFUSE,
    });
    spheres
}

// N is a surface normal (a unit vector pointing from center to surface)
// we just give each a nice unique color so that we can discern the surface
#[allow(dead_code)]
fn color_surface_normal(normal: &Vec3) -> Color {
    0.5 * Color::new(normal.x + 1., normal.y + 1., normal.z + 1.)
}

fn color_hit(scene: &dyn Hittable, ray: &Ray, hit: &Hit, remaining_bounces: usize) -> Color {
    if remaining_bounces <= 0 {
        return Color::black();
    }
    match hit.material.scatter(ray, hit) {
        Some((scattered, attenuation)) => {
            attenuation * ray_color(&scattered, scene, remaining_bounces - 1)
        }
        None => Color::black(),
    }
}

// blue to white grandient based on y
fn color_gradient_background(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.unit().y + 1.);
    t * Color::new(0.5, 0.7, 1.0) + (1. - t) * Color::new(1., 1., 1.)
}

fn ray_color(ray: &Ray, scene: &dyn Hittable, remaining_bounces: usize) -> Color {
    match scene.hit(ray, 0.001, INFINITY) {
        Some(hit) => color_hit(scene, ray, &hit, remaining_bounces),
        None => color_gradient_background(ray),
    }
}

fn main() -> Result<(), std::io::Error> {
    let canvas = Canvas {
        width: 500,
        height: 300,
    };
    let samples_per_pixel = 100;

    let mut writer = PpmWriter::new(BufWriter::new(File::create("out.ppm")?), &canvas);
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
