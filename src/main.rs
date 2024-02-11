mod ppm_writer;

use keyell::render::{
    Background, Camera, Canvas, Color, Colorer, Degrees, Dielectric, Diffuse, Light, Metal, Plane,
    Sphere,
};
use keyell::types::{Normal, Point, Vec3};
use keyell::Scene;

use ppm_writer::PpmWriter;
use std::fs::File;
use std::io::BufWriter;

fn make_scene() -> Scene<'static> {
    const DIFFUSE: Diffuse = Diffuse {
        colorer: Colorer::Solid(Color::new(0.9, 0.2, 0.3)),
    };

    const WHITE_DIELECTRIC: Dielectric = Dielectric {
        refraction_index: 1.3,
        colorer: Colorer::Solid(Color::WHITE),
    };

    const PURPLE_DIELECTRIC: Dielectric = Dielectric {
        refraction_index: 0.4,
        colorer: Colorer::Solid(Color::new(0.6, 0.3, 0.9)),
    };

    const METAL: Metal = Metal {
        colorer: Colorer::Solid(Color::new(1., 1., 1.)),
        fuzz: 0.0,
    };

    let spheres = vec![
        Sphere {
            center: Point::new(0., 1., 0.),
            radius: 0.7,
            material: &Diffuse {
                colorer: Colorer::Bubblegum,
            },
        },
        Sphere {
            center: Point::new(0.2, 0.26, 0.),
            radius: 0.1,
            material: &DIFFUSE,
        },
        Sphere {
            center: Point::new(0.03, 0.25, 0.1),
            radius: 0.05,
            material: &WHITE_DIELECTRIC,
        },
        Sphere {
            center: Point::new(-0.05, 0.2, 0.07),
            radius: 0.05,
            material: &PURPLE_DIELECTRIC,
        },
        Sphere {
            center: Point::new(0., -0.5, 0.),
            radius: 0.3,
            material: &Light {
                colorer: Colorer::Bubblegum,
            },
        },
        Sphere {
            center: Point::new(0.1, 0.3, 0.1),
            radius: 0.1,
            material: &METAL,
        },
    ];

    const GREEN_DIFFUSE: Diffuse = Diffuse {
        colorer: Colorer::Solid(Color::new(0.4, 0.8, 0.4)),
    };

    let planes = vec![Plane {
        point: Point::new(0., 0., 0.),
        normal: Normal::Outward(Vec3::new(0., 0., 1.).unit()),
        material: &GREEN_DIFFUSE,
    }];

    const GRADIENT: Light = Light {
        colorer: Colorer::ZGradient {
            top: Color::new(0.5, 0.7, 1.0),
            bottom: Color::BLACK,
        },
    };
    const BACKGROUND: Background = Background {
        material: &GRADIENT,
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

    let mut writer = PpmWriter::new(BufWriter::new(File::create("out.ppm")?), &CANVAS);
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
        writer.write_pixel(&pixel)?;
    }

    Ok(())
}
