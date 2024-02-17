use std::{
    convert::TryFrom,
    fs::File,
    io::{BufWriter, Read, Write},
    net::TcpStream,
    ops::Range,
    sync::Arc,
};

use keyell::{
    net::Request,
    render::{Background, Camera, Canvas, Color, Colorer, Material, Sphere},
    types::Point,
    Scene,
};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

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

    let canvas = Arc::new(Canvas {
        width: 1920,
        height: 1080,
    });
    let camera = Arc::new(Camera::from_canvas(
        &canvas,
        keyell::types::Point::new(0., 0., 0.05),
        keyell::render::Degrees::new(90.),
    ));

    let mut pixels = vec![0u8; 3 * canvas.width * canvas.height];

    struct RequestParams<'a> {
        ip: &'a str,
        range: Range<usize>,
        pixels: &'a mut [u8],
    }

    let cutoff = 800;
    let (first, second) = pixels.split_at_mut(3 * cutoff * canvas.width);
    let mut params = [
        RequestParams {
            ip: "192.168.1.129:3544",
            range: 0..cutoff,
            pixels: first,
        },
        RequestParams {
            ip: "127.0.0.1:3544",
            range: cutoff..(canvas.height),
            pixels: second,
        },
    ];

    let scene = Arc::new(scene);
    params.par_iter_mut().enumerate().for_each(|(i, params)| {
        let mut stream = TcpStream::connect(params.ip).unwrap();

        let request = Request {
            scene: scene.clone(),
            canvas: canvas.clone(),
            camera: camera.clone(),
            samples_per_pixel: 100,
            maximum_bounces: 10,
            range: params.range.clone(),
        };

        let mut serialized = Vec::new();
        serde_json::to_writer(&mut serialized, &request).unwrap();
        stream.write(&serialized.len().to_le_bytes()).unwrap();
        stream.write_all(&serialized).unwrap();
        stream.flush().unwrap();
        println!("wrote request {i}");
        stream.read_exact(params.pixels).unwrap();
        println!("got response {i}");
    });

    let mut writer =
        keyell::ppm::PpmWriter::new(BufWriter::new(File::create("client.ppm")?), &canvas);
    writer.write_header()?;
    for pixel in pixels.chunks_exact(3) {
        writer.write_pixel(<&[u8; 3]>::try_from(pixel).unwrap())?;
    }
    println!("wrote client.ppm");

    Ok(())
}
