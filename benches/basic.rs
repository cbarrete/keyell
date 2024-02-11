use criterion::{black_box, criterion_group, criterion_main, Criterion};
use keyell::{
    render::{
        Background, Camera, Canvas, Color, Colorer, Degrees, Dielectric, Diffuse, Light, Metal,
        Plane, Sphere,
    },
    render_scene,
    types::{Normal, Point, Vec3},
    Scene,
};

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
