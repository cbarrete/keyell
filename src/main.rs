use std::fs::File;

mod types;
mod ppm_writer;

use crate::ppm_writer::PPMWriter;
use crate::types::Vec3;

fn main() -> Result<(), std::io::Error> {
    let width  = 200;
    let height = 100;

    let mut writer = PPMWriter::new(File::create("out.ppm")?, width, height);
    writer.write_header()?;

    for j in (0..height).rev() {
        for i in 0..width {
            let r = i as f64 / width as f64;
            let g = j as f64 / height as f64;
            let b = 0.2 as f64;
            writer.write(&Vec3 { x: r, y: g, z: b })?;
        }
    }

    Ok(())
}
