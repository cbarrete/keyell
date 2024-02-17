use std::{
    io::{BufWriter, Read, Write},
    net::TcpListener,
};

use keyell::{net::Request, render::Color};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3544")?;
    println!("listening");
    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut len = [0u8; 8];
        stream.read_exact(&mut len)?;
        let mut buffer = vec![0; usize::from_le_bytes(len)];
        stream.read_exact(&mut buffer)?;

        let request: Request = serde_json::from_reader(buffer.as_slice()).unwrap();
        let mut pixels = vec![Color::WHITE; request.canvas.width * request.range.len()];
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

        let mut writer = BufWriter::new(&mut stream);
        for pixel in &pixels {
            writer.write(&[
                (255.999 * pixel.r).floor() as u8,
                (255.999 * pixel.g).floor() as u8,
                (255.999 * pixel.b).floor() as u8,
            ])?;
        }
        writer.flush()?;
        println!("wrote {} bytes to the client", 3 * pixels.len());
    }
    Ok(())
}
