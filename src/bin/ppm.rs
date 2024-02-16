use keyell::render::{
    Background, Camera, Canvas, Color, Colorer, Degrees, Material, Plane, Sphere,
};
use keyell::types::{Normal, Point, Vec3};
use keyell::Scene;

use std::fs::File;
use std::io::BufWriter;

fn make_scene() -> Scene {
    let spheres = vec![
        Sphere {
            center: Point::new(0., 1., 0.),
            radius: 0.7,
            material: Material::Diffuse(Colorer::Bubblegum),
        },
        Sphere {
            center: Point::new(0.2, 0.26, 0.),
            radius: 0.1,
            material: Material::Diffuse(Colorer::Solid(Color::new(0.9, 0.2, 0.3))),
        },
        Sphere {
            center: Point::new(0.03, 0.25, 0.1),
            radius: 0.05,
            material: Material::Dielectric {
                refraction_index: 1.3,
                colorer: Colorer::Solid(Color::WHITE),
            },
        },
        Sphere {
            center: Point::new(-0.05, 0.2, 0.07),
            radius: 0.05,
            material: Material::Dielectric {
                refraction_index: 0.4,
                colorer: Colorer::Solid(Color::new(0.6, 0.3, 0.9)),
            },
        },
        Sphere {
            center: Point::new(0., -0.5, 0.),
            radius: 0.3,
            material: Material::Light(Colorer::Bubblegum),
        },
        Sphere {
            center: Point::new(0.1, 0.3, 0.1),
            radius: 0.1,
            material: Material::Metal {
                colorer: Colorer::Solid(Color::new(1., 1., 1.)),
                fuzz: 0.0,
            },
        },
    ];

    let planes = vec![Plane {
        point: Point::new(0., 0., 0.),
        normal: Normal::Outward(Vec3::new(0., 0., 1.).unit()),
        material: Material::Diffuse(Colorer::Solid(Color::new(0.4, 0.8, 0.4))),
    }];

    const BACKGROUND: Background = Background {
        material: Material::Light(Colorer::ZGradient {
            top: Color::new(0.5, 0.7, 1.0),
            bottom: Color::BLACK,
        }),
    };

    Scene {
        spheres,
        planes,
        background: BACKGROUND,
    }
}

fn main() -> Result<(), std::io::Error> {
    const CANVAS: Canvas = Canvas {
        width: 1920,
        height: 1080,
    };
    let samples_per_pixel = 10;
    let maximum_bounces = 10;

    let mut writer = keyell::ppm::PpmWriter::new(BufWriter::new(File::create("out.ppm")?), &CANVAS);
    writer.write_header()?;

    let camera = Camera::from_canvas(&CANVAS, Point::new(0., 0., 0.05), Degrees::new(90.));
    let scene = make_scene();
    let mut pixels = vec![Color::BLACK; CANVAS.width * CANVAS.height];

    let begin = std::time::Instant::now();
    keyell::render_scene(
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
        writer.write_color(&pixel)?;
    }

    Ok(())
}
