mod math;
mod physics;
pub mod render;
pub mod types;

use render::{
    Background, Bounce, Camera, Canvas, Color, Hit, Hittable, Interaction, Plane, Ray, Source,
    Sphere,
};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::f32::INFINITY;

// TODO: don't put this here?
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
    pub background: Background,
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest_travel = t_max;
        let mut closest_hit = self.background.hit(ray, t_min, closest_travel);

        for sphere in &self.spheres {
            if let Some(hit) = sphere.hit(ray, t_min, closest_travel) {
                closest_travel = hit.travel;
                closest_hit = Some(hit);
            }
        }

        for plane in &self.planes {
            if let Some(hit) = plane.hit(ray, t_min, closest_travel) {
                closest_travel = hit.travel;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}

fn color_hit(
    scene: &Scene,
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

fn ray_color(ray: &Ray, scene: &Scene, remaining_bounces: usize, rng: &mut SmallRng) -> Color {
    match scene.hit(ray, 0.001, INFINITY) {
        Some(hit) => color_hit(scene, ray, &hit, remaining_bounces, rng),
        None => Color::BLACK,
    }
}

pub fn render_scene(
    pixels: &mut [Color],
    scene: &Scene,
    canvas: &Canvas,
    camera: &Camera,
    samples_per_pixel: usize,
    maximum_bounces: usize,
) {
    let mut rngs: Vec<SmallRng> = (0..canvas.height)
        .map(|i| SmallRng::seed_from_u64(i as u64))
        .collect();

    pixels
        .chunks_mut(canvas.width)
        .enumerate()
        .zip(&mut rngs)
        .par_bridge()
        .for_each(|((j, colors), rng)| {
            let j = canvas.height - j - 1;
            for i in 0..canvas.width {
                let mut color = Color::BLACK;
                for _ in 0..samples_per_pixel {
                    let u = (rng.gen_range(0. ..1.) + i as f32) / canvas.width as f32;
                    let v = (rng.gen_range(0. ..1.) + j as f32) / canvas.height as f32;
                    color = color + ray_color(&camera.get_ray(u, v), scene, maximum_bounces, rng);
                }
                colors[i] = color / samples_per_pixel as f32;
            }
        });
}
