mod math;
mod physics;
mod ppm_writer;
mod render;
mod types;

use ppm_writer::PpmWriter;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use render::{
    Background, Bounce, Bubblegum, Camera, Canvas, Color, Degrees, Dielectric, Diffuse, Hit,
    Hittable, Interaction, Light, Metal, Plane, Ray, Solid, Source, Sphere, ZGradient,
};
use std::fs::File;
use std::{f32::INFINITY, io::BufWriter};
use types::{Normal, Point, Vec3};

type Scene = Vec<Box<dyn Hittable>>;

fn make_scene() -> Scene {
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

fn color_hit(
    scene: &dyn Hittable,
    ray: &Ray,
    hit: &Hit,
    remaining_bounces: usize,
    rng: &mut SmallRng,
) -> Color {
    if remaining_bounces == 0 {
        return Color::BLACK;
    }
    match hit.material.scatter(ray, hit, rng) {
        Interaction::Bounce(Bounce {
            scattered,
            attenuation,
        }) => attenuation * ray_color(&scattered, scene, remaining_bounces - 1, rng),
        Interaction::Source(Source { color }) => color,
        Interaction::Nothing => Color::BLACK,
    }
}

fn ray_color(
    ray: &Ray,
    scene: &dyn Hittable,
    remaining_bounces: usize,
    rng: &mut SmallRng,
) -> Color {
    match scene.hit(ray, 0.001, INFINITY) {
        Some(hit) => color_hit(scene, ray, &hit, remaining_bounces, rng),
        None => Color::BLACK,
    }
}

fn render_scene(
    pixels: &mut [Color],
    scene: &Scene,
    canvas: &Canvas,
    camera: &Camera,
    samples_per_pixel: usize,
    maximum_bounces: usize,
) {
    let mut rng = SmallRng::seed_from_u64(0);

    let mut pixel_index = 0;
    for j in (0..canvas.height).rev() {
        for i in 0..canvas.width {
            let mut color = Color::BLACK;
            for _ in 0..samples_per_pixel {
                let u = (rng.gen_range(0. ..1.) + i as f32) / canvas.width as f32;
                let v = (rng.gen_range(0. ..1.) + j as f32) / canvas.height as f32;
                color = color + ray_color(&camera.get_ray(u, v), scene, maximum_bounces, &mut rng);
            }
            pixels[pixel_index] = color / samples_per_pixel as f32;
            pixel_index += 1;
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    const CANVAS: Canvas = Canvas {
        width: 1920,
        height: 1080,
    };
    let samples_per_pixel = 10;
    let maximum_bounces = 10;

    let mut writer = PpmWriter::new(BufWriter::new(File::create("out.ppm")?), &CANVAS);
    writer.write_header()?;

    let camera = Camera::from_canvas(&CANVAS, Point::new(0., 0., 0.05), Degrees::new(90.));
    let scene = make_scene();
    let mut pixels = vec![Color::BLACK; CANVAS.width * CANVAS.height];

    let begin = std::time::Instant::now();
    render_scene(
        &mut pixels,
        &scene,
        &CANVAS,
        &camera,
        samples_per_pixel,
        maximum_bounces,
    );
    let duration = begin.elapsed();
    dbg!(duration);

    for pixel in pixels {
        writer.write_pixel(&pixel)?;
    }

    Ok(())
}
