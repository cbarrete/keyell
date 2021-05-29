mod math;
mod physics;
mod ppm_writer;
mod types;

use ppm_writer::PpmWriter;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::{f64::INFINITY, io::BufWriter};
use types::{
    Background, Bounce, Bubblegum, Camera, Canvas, Color, Degrees, Dielectric, Diffuse, Hit,
    Hittable, Interaction, Light, Metal, Point, Ray, Solid, Source, Sphere, ZGradient,
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

const LIGHT: Light = Light {
    colorer: &Bubblegum {},
};

pub struct HitTable<'a> {
    pub spheres: Vec<Sphere<'a>>,
    pub background: Option<Background<'a>>,
}

impl<'a> Hittable for HitTable<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let sphere_hit = self.spheres.hit(ray, t_min, t_max);
        if sphere_hit.is_some() {
            return sphere_hit;
        }
        if let Some(bg) = &self.background {
            return bg.hit(ray, t_min, t_max);
        }
        None
    }
}

fn make_scene() -> HitTable<'static> {
    let mut spheres: Vec<Sphere<'static>> = Vec::new();
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
        center: Point::new(-0.08, -0.5, 0.),
        radius: 0.3,
        material: &LIGHT,
    });
    spheres.push(Sphere {
        center: Point::new(0.1, 0.3, 0.),
        radius: 0.1,
        material: &STEEL,
    });

    const GRADIENT: Light = Light {
        colorer: &ZGradient {
            top: Color::new(0.5, 0.7, 1.0),
            bottom: Color::BLACK,
        },
    };
    let background = Some(Background {
        material: &GRADIENT,
    });

    HitTable {
        spheres,
        background,
    }
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

    let camera = Camera::from_canvas(&canvas, Point::new(0., 0., 0.), Degrees::new(90.));
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
