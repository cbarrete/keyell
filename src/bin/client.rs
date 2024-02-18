use std::{fs::File, io::BufWriter};

use keyell::{
    net::{render_scene_distributed, Remote},
    render::{Background, Camera, Canvas, Color, Colorer, Material, Sphere},
    types::Point,
    Scene,
};

fn main() -> std::io::Result<()> {
    let mut scene = Scene {
        spheres: Vec::new(),
        planes: Vec::new(),
        background: Background {
            material: Material::Light(Colorer::ZGradient {
                bottom: Color::WHITE,
                top: Color::new(0.4, 0.3, 0.8),
            }),
        },
    };

    for i in 0..11 {
        scene.spheres.push(Sphere {
            center: Point::new(-1. + (2. * i as f32 / 10.), 1., 0.),
            radius: 0.1,
            material: Material::Light(Colorer::Solid(Color::new(0.4, 0.6, 0.9))),
        });
    }

    let canvas = Canvas {
        width: 1920,
        height: 1080,
    };
    let camera = Camera::from_canvas(
        &canvas,
        keyell::types::Point::new(0., 0., 0.05),
        keyell::render::Degrees::new(90.),
    );

    let mut pixels = vec![Color::BLACK; canvas.width * canvas.height];
    render_scene_distributed(
        &[
            // Remote {
            //     ip: String::from("127.0.0.1:3544"),
            //     rows: canvas.height / 4,
            // },
            Remote {
                ip: String::from("192.168.1.129:3544"),
                rows: 3 * canvas.height / 4,
            },
        ],
        &mut pixels,
        &scene,
        &canvas,
        &camera,
        100,
        10,
    );

    let mut writer =
        keyell::ppm::PpmWriter::new(BufWriter::new(File::create("client.ppm")?), &canvas);
    writer.write_header()?;
    for pixel in &pixels {
        writer.write_pixel(pixel).unwrap();
    }
    println!("wrote client.ppm");

    Ok(())
}
