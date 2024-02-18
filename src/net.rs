use std::{
    io::{Read, Write},
    net::TcpStream,
    ops::Range,
    sync::Arc,
};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    render::{Camera, Canvas},
    Scene,
};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub scene: Arc<Scene>,
    pub canvas: Arc<Canvas>,
    pub camera: Arc<Camera>,
    pub samples_per_pixel: usize,
    pub maximum_bounces: usize,
    pub range: Range<usize>,
}

pub fn render_scene_distributed(
    ips: (&str, &str),
    cutoff: usize,
    pixels: &mut [u8],
    scene: Arc<Scene>,
    canvas: Arc<Canvas>,
    camera: Arc<Camera>,
    samples_per_pixel: usize,
    maximum_bounces: usize,
) {
    debug_assert!((0..(canvas.height)).contains(&cutoff));

    struct RequestParams<'a> {
        ip: &'a str,
        range: Range<usize>,
        pixels: &'a mut [u8],
    }

    let (first, second) = pixels.split_at_mut(3 * cutoff * canvas.width);
    let mut params = [
        RequestParams {
            ip: ips.0,
            range: 0..cutoff,
            pixels: first,
        },
        RequestParams {
            ip: ips.1,
            range: cutoff..(canvas.height),
            pixels: second,
        },
    ];

    params.par_iter_mut().enumerate().for_each(|(i, params)| {
        let mut stream = TcpStream::connect(params.ip).unwrap();

        let request = Request {
            scene: scene.clone(),
            canvas: canvas.clone(),
            camera: camera.clone(),
            samples_per_pixel,
            maximum_bounces,
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
}
