use std::{
    io::{Read, Write},
    net::TcpListener,
};

use keyell::{net::Request, render::Color};

fn main() -> std::io::Result<()> {
    let mut pixels = Vec::new();

    let listener = TcpListener::bind("0.0.0.0:3544")?;
    println!("listening");

    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut len = [0u8; 8];
        stream.read_exact(&mut len)?;
        let mut buffer = vec![0; usize::from_le_bytes(len)];
        stream.read_exact(&mut buffer)?;

        let request: Request = serde_json::from_reader(buffer.as_slice()).unwrap();
        pixels.resize(request.canvas.width * request.range.len(), Color::BLACK);
        println!("rendering...");
        keyell::render_scene(
            &mut pixels,
            &request.scene,
            &request.canvas,
            &request.camera,
            request.samples_per_pixel,
            request.maximum_bounces,
            request.range,
        );
        println!("rendered");

        let bytes_ptr = pixels.as_ptr() as *const u8;
        let bytes_len = std::mem::size_of::<Color>() * pixels.len();
        let bytes = unsafe { std::slice::from_raw_parts(bytes_ptr, bytes_len) };
        stream.write(bytes)?;
        println!("wrote {} bytes to the client", bytes.len());
    }
    Ok(())
}
