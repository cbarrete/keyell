use rand::{thread_rng, Rng};
use std::fs::File;
use std::{f64::INFINITY, io::BufWriter};
use types::Bubblegum;
use types::Solid;

mod math;
mod physics;
mod ppm_writer;
mod types;

use crate::ppm_writer::PpmWriter;
use crate::types::{
    Bounce, Camera, Canvas, Color, Dielectric, Diffuse, Hit, Hittable, Metal, Point, Ray, Sphere,
};

const BBG_DIFFUSE: Diffuse = Diffuse {
    colorer: &Bubblegum {},
};
const RED_DIFFUSE: Diffuse = Diffuse {
    colorer: &Solid::from_color(Color::new(0.9, 0.2, 0.3)),
};
const GREEN_DIFFUSE: Diffuse = Diffuse {
    colorer: &Solid::from_color(Color::new(0.4, 0.8, 0.4)),
};
const TINTED_METAL: Metal = Metal {
    colorer: &Bubblegum {},
    fuzz: 0.3,
};
const STEEL: Metal = Metal {
    colorer: &Solid::from_color(Color::new(1., 1., 1.)),
    fuzz: 0.0,
};
const HIGH_DIALECTRIC: Dielectric = Dielectric {
    refraction_index: 1.3,
    colorer: &Solid::from_color(Color::WHITE),
};
const LOW_DIALECTRIC: Dielectric = Dielectric {
    refraction_index: 0.3,
    colorer: &Solid::from_color(Color::new(0.6, 0.3, 0.9)),
};

fn make_spheres() -> Vec<Sphere<'static>> {
    let mut spheres = Vec::with_capacity(4);
    spheres.push(Sphere {
        center: Point::new(0., 0., -100.1),
        radius: 100.,
        material: &GREEN_DIFFUSE,
    });
    spheres.push(Sphere {
        center: Point::new(0., 1., 0.),
        radius: 0.7,
        material: &BBG_DIFFUSE,
    });
    spheres.push(Sphere {
        center: Point::new(0.2, 0.26, 0.),
        radius: 0.1,
        material: &RED_DIFFUSE,
    });
    spheres.push(Sphere {
        center: Point::new(-0.4, 0.5, 0.5),
        radius: 0.3,
        material: &TINTED_METAL,
    });
    spheres.push(Sphere {
        center: Point::new(0.03, 0.25, 0.1),
        radius: 0.05,
        material: &HIGH_DIALECTRIC,
    });
    spheres.push(Sphere {
        center: Point::new(-0.05, 0.2, 0.),
        radius: 0.05,
        material: &LOW_DIALECTRIC,
    });
    spheres.push(Sphere {
        center: Point::new(0.1, 0.3, 0.),
        radius: 0.1,
        material: &STEEL,
    });
    spheres.push(Sphere {
        center: Point::new(0., -3., 0.),
        radius: 0.5,
        material: &RED_DIFFUSE,
    });
    spheres
}

fn color_hit(scene: &dyn Hittable, ray: &Ray, hit: &Hit, remaining_bounces: usize) -> Color {
    if remaining_bounces <= 0 {
        return Color::BLACK;
    }
    match hit.material.scatter(ray, hit) {
        Some(Bounce {
            scattered,
            attenuation,
        }) => attenuation * ray_color(&scattered, scene, remaining_bounces - 1),
        None => Color::BLACK,
    }
}

// blue to white grandient based on z
fn color_gradient_background(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.unit().get().z + 1.);
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
    let samples_per_pixel = 50;

    let mut writer = PpmWriter::new(BufWriter::new(File::create("out.ppm")?), &canvas);
    writer.write_header()?;

    let camera = Camera::from_canvas(&canvas);
    let scene = make_spheres();

    let mut rng = thread_rng();

    for j in (0..canvas.height).rev() {
        for i in 0..canvas.width {
            let mut color = Color::BLACK;
            for _ in 0..samples_per_pixel {
                let u = (rng.gen_range(0., 1.) + i as f64) / canvas.width as f64;
                let v = (rng.gen_range(0., 1.) + j as f64) / canvas.height as f64;
                color = color + ray_color(&camera.get_ray(u, v), &scene, 50);
            }
            writer.write_pixel(&(color / samples_per_pixel as f64))?;
        }
    }

    Ok(())
}
