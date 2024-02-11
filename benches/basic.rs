use criterion::{black_box, criterion_group, criterion_main, Criterion};
use keyell::{
    render::{Background, Camera, Canvas, Color, Colorer, Degrees, Material, Plane, Sphere},
    render_scene,
    types::{Normal, Point, Vec3},
    Scene,
};

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

fn benchmarked(pixels: &mut [Color], scene: &Scene, canvas: &Canvas) {
    let samples_per_pixel = 10;
    let maximum_bounces = 10;
    let camera = Camera::from_canvas(canvas, Point::new(0., 0., 0.05), Degrees::new(90.));
    render_scene(
        pixels,
        scene,
        canvas,
        &camera,
        samples_per_pixel,
        maximum_bounces,
    );
}

fn benchmark(c: &mut Criterion) {
    const CANVAS: Canvas = Canvas {
        width: 100,
        height: 100,
    };
    let mut pixels = vec![Color::BLACK; CANVAS.width * CANVAS.height];
    let scene = make_scene();
    c.bench_function("test", |b| {
        b.iter(|| benchmarked(black_box(&mut pixels), black_box(&scene), &CANVAS))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
