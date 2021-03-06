mod math;
mod physics;
mod ppm_writer;
mod render;
mod types;

use ppm_writer::PpmWriter;
use rand::{thread_rng, Rng};
use render::{
    Background, Bounce, Bubblegum, Camera, Canvas, Color, Degrees, Dielectric, Diffuse, Hit,
    Hittable, Interaction, Light, Metal, Plane, Ray, Solid, Source, Sphere, ZGradient,
};
use std::fs::File;
use std::{f64::INFINITY, io::BufWriter};
use types::{Normal, Point, Vec3};

fn make_scene() -> Vec<Box<dyn Hittable>> {
    const SPHERES: [Sphere<'static>; 6] = [
        Sphere {
            center: Point::new(0., 1., 0.),
            radius: 0.7,
            material: &Diffuse {
                colorer: &Bubblegum {},
            },
        },
        Sphere {
            center: Point::new(0.2, 0.26, 0.),
            radius: 0.1,
            material: &Diffuse {
                colorer: &Solid::from_color(Color::new(0.9, 0.2, 0.3)),
            },
        },
        Sphere {
            center: Point::new(0.03, 0.25, 0.1),
            radius: 0.05,
            material: &Dielectric {
                refraction_index: 1.3,
                colorer: &Solid::from_color(Color::WHITE),
            },
        },
        Sphere {
            center: Point::new(-0.05, 0.2, 0.07),
            radius: 0.05,
            material: &Dielectric {
                refraction_index: 0.4,
                colorer: &Solid::from_color(Color::new(0.6, 0.3, 0.9)),
            },
        },
        Sphere {
            center: Point::new(0., -0.5, 0.),
            radius: 0.3,
            material: &Light {
                colorer: &Bubblegum {},
            },
        },
        Sphere {
            center: Point::new(0.1, 0.3, 0.1),
            radius: 0.1,
            material: &Metal {
                colorer: &Solid::from_color(Color::new(1., 1., 1.)),
                fuzz: 0.0,
            },
        },
    ];

    const GREEN_DIFFUSE: Diffuse = Diffuse {
        colorer: &Solid::from_color(Color::new(0.4, 0.8, 0.4)),
    };

    let planes = vec![Plane {
        point: Point::new(0., 0., 0.),
        normal: Normal::Outward(Vec3::new(0., 0., 1.).unit()),
        material: &GREEN_DIFFUSE,
    }];

    const GRADIENT: Light = Light {
        colorer: &ZGradient {
            top: Color::new(0.5, 0.7, 1.0),
            bottom: Color::BLACK,
        },
    };
    const BACKGROUND: Background = Background {
        material: &GRADIENT,
    };

    vec![Box::new(SPHERES), Box::new(planes), Box::new(BACKGROUND)]
}

fn color_hit(scene: &dyn Hittable, ray: &Ray, hit: &Hit, remaining_bounces: usize) -> Color {
    if remaining_bounces == 0 {
        return Color::BLACK;
    }
    match hit.material.scatter(ray, hit) {
        Interaction::Bounce(Bounce {
            scattered,
            attenuation,
        }) => attenuation * ray_color(&scattered, scene, remaining_bounces - 1),
        Interaction::Source(Source { color }) => color,
        Interaction::Nothing => Color::BLACK,
    }
}

fn ray_color(ray: &Ray, scene: &dyn Hittable, remaining_bounces: usize) -> Color {
    match scene.hit(ray, 0.001, INFINITY) {
        Some(hit) => color_hit(scene, ray, &hit, remaining_bounces),
        None => Color::BLACK,
    }
}

fn main() -> Result<(), std::io::Error> {
    let canvas = Canvas {
        width: 500,
        height: 300,
    };
    let samples_per_pixel = 50;
    let maximum_bounces = 50;

    let mut writer = PpmWriter::new(BufWriter::new(File::create("out.ppm")?), &canvas);
    writer.write_header()?;

    let camera = Camera::from_canvas(&canvas, Point::new(0., 0., 0.05), Degrees::new(90.));
    let scene = make_scene();

    let mut rng = thread_rng();

    for j in (0..canvas.height).rev() {
        for i in 0..canvas.width {
            let mut color = Color::BLACK;
            for _ in 0..samples_per_pixel {
                let u = (rng.gen_range(0., 1.) + i as f64) / canvas.width as f64;
                let v = (rng.gen_range(0., 1.) + j as f64) / canvas.height as f64;
                color = color + ray_color(&camera.get_ray(u, v), &scene, maximum_bounces);
            }
            writer.write_pixel(&(color / samples_per_pixel as f64))?;
        }
    }

    Ok(())
}
