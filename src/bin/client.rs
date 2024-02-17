use std::convert::TryFrom;
use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    net::TcpStream,
};

use keyell::{
    net::Request,
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
    let h = canvas.height;
    let w = canvas.width;
    let mut request = Request {
        scene,
        canvas,
        camera,
        samples_per_pixel: 1,
        maximum_bounces: 1,
        range: 0..(h / 2),
    };

    let mut pixels = vec![0u8; 3 * request.canvas.width * request.canvas.height];

    {
        let mut stream = TcpStream::connect("192.168.1.129:3544")?;
        let mut serialized = Vec::new();
        serde_json::to_writer(&mut serialized, &request).unwrap();
        stream.write(&serialized.len().to_le_bytes())?;
        stream.write_all(&serialized).unwrap();
        stream.flush().unwrap();
        println!("wrote first request");
        stream.read_exact(&mut pixels[3 * w * h / 2..]).unwrap();
        println!("got first response");
    }

    {
        let mut stream = TcpStream::connect("127.0.0.1:3544")?;
        request.range = (h / 2)..h;
        let mut serialized = Vec::new();
        serde_json::to_writer(&mut serialized, &request).unwrap();
        stream.write(&serialized.len().to_le_bytes()).unwrap();
        stream.write_all(&serialized).unwrap();
        stream.flush().unwrap();
        println!("wrote second request");
        stream.read_exact(&mut pixels[0..3 * w * h / 2]).unwrap();
        println!("got first response");
    }

    let mut writer =
        keyell::ppm::PpmWriter::new(BufWriter::new(File::create("client.ppm")?), &request.canvas);
    writer.write_header()?;
    for pixel in pixels.chunks_exact(3) {
        writer.write_pixel(<&[u8; 3]>::try_from(pixel).unwrap())?;
    }
    println!("wrote client.ppm");

    Ok(())
}
