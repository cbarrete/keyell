use std::{
    io::{Read, Write},
    net::TcpStream,
    ops::Range,
    sync::Arc,
};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    render::{Camera, Canvas, Color},
    render_scene, Scene,
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

pub struct Remote<'a> {
    pub ip: &'a str,
    pub rows: usize,
}

pub fn render_scene_distributed(
    remotes: &[Remote],
    pixels: &mut [Color],
    mut buffer: &mut [u8],
    scene: Arc<Scene>,
    canvas: Arc<Canvas>,
    camera: Arc<Camera>,
    samples_per_pixel: usize,
    maximum_bounces: usize,
) {
    for remote in remotes {
        debug_assert!((0..(canvas.height)).contains(&remote.rows));
    }

    // TODO: remove copies between buffer and pixels
    // - make Color repr(C)
    // - send floats on the network (more traffic but more precision as well)
    let local_rows = canvas.height - remotes.iter().map(|r| r.rows).sum::<usize>();
    let (local_pixels, mut pixels) = pixels.split_at_mut(local_rows * canvas.width);

    struct RequestParams<'a> {
        ip: &'a str,
        range: Range<usize>,
        buffer: &'a mut [u8],
        pixels: &'a mut [Color],
    }

    let mut params = Vec::new();
    let mut start = local_rows;
    for remote in remotes {
        let (current_buffer, remaining_buffer) =
            buffer.split_at_mut(3 * remote.rows * canvas.width);
        let (current_pixels, remaining_pixels) = pixels.split_at_mut(remote.rows * canvas.width);
        buffer = remaining_buffer;
        pixels = remaining_pixels;
        params.push(RequestParams {
            ip: remote.ip,
            range: start..(start + remote.rows),
            buffer: current_buffer,
            pixels: current_pixels,
        });
        start += remote.rows;
    }

    std::thread::scope(|s| {
        s.spawn(|| {
            render_scene(
                local_pixels,
                &scene,
                &canvas,
                &camera,
                samples_per_pixel,
                maximum_bounces,
                0..local_rows,
            );
        });

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
            stream.read_exact(params.buffer).unwrap();
            println!("got response {i}");
            for (bytes, pixel) in params.buffer.chunks_exact(3).zip(params.pixels.iter_mut()) {
                *pixel = Color::new(
                    bytes[0] as f32 / 255.,
                    bytes[1] as f32 / 255.,
                    bytes[2] as f32 / 255.,
                );
            }
        });
    });
}
