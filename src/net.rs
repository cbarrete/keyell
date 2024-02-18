use std::{
    io::{Read, Write},
    net::TcpStream,
    ops::Range,
};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    render::{Camera, Canvas, Color},
    render_scene, Scene,
};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub scene: Scene,
    pub canvas: Canvas,
    pub camera: Camera,
    pub samples_per_pixel: usize,
    pub maximum_bounces: usize,
    pub range: Range<usize>,
}

pub struct Remote {
    pub ip: String,
    pub rows: usize,
}

pub fn render_scene_distributed(
    remotes: &[Remote],
    pixels: &mut [Color],
    scene: &Scene,
    canvas: &Canvas,
    camera: &Camera,
    samples_per_pixel: usize,
    maximum_bounces: usize,
) {
    for remote in remotes {
        debug_assert!((0..(canvas.height)).contains(&remote.rows));
    }

    let local_rows = canvas.height - remotes.iter().map(|r| r.rows).sum::<usize>();
    let (local_pixels, mut pixels) = pixels.split_at_mut(local_rows * canvas.width);

    struct RequestParams<'a> {
        ip: &'a str,
        range: Range<usize>,
        pixels: &'a mut [Color],
    }

    let mut params = Vec::new();
    let mut start = local_rows;
    for remote in remotes {
        let (current_pixels, remaining_pixels) = pixels.split_at_mut(remote.rows * canvas.width);
        pixels = remaining_pixels;
        params.push(RequestParams {
            ip: &remote.ip,
            range: start..(start + remote.rows),
            pixels: current_pixels,
        });
        start += remote.rows;
    }

    std::thread::scope(|s| {
        s.spawn(|| {
            println!("rendering locally...");
            render_scene(
                local_pixels,
                &scene,
                &canvas,
                &camera,
                samples_per_pixel,
                maximum_bounces,
                0..local_rows,
            );
            println!("done rendering locally");
        });

        params.par_iter_mut().enumerate().for_each(|(i, params)| {
            let mut stream = TcpStream::connect(params.ip).unwrap();

            // TODO: Not happy with all those copies.
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
            let bytes_ptr = params.pixels.as_ptr() as *mut u8;
            let bytes_len = std::mem::size_of::<Color>() * params.pixels.len();
            let bytes = unsafe { std::slice::from_raw_parts_mut(bytes_ptr, bytes_len) };
            stream.read_exact(bytes).unwrap();
            println!("got response {i}");
        });
    });
}
