use std::fs::File;

mod types;
mod ppm_writer;

use crate::ppm_writer::PPMWriter;
use crate::types::{Ray, Vec3};

// blend blue and white, used to make a gradient background.
fn ray_color(ray: &Ray) -> Vec3 {
    let t = 0.5 * (ray.direction.unit().y + 1.);
    t * Vec3::new(0.5, 0.7, 1.0) + (1. - t) * Vec3::new(1., 1., 1.)
}

fn main() -> Result<(), std::io::Error> {
    let width  = 200;
    let height = 100;

    let mut writer = PPMWriter::new(File::create("out.ppm")?, width, height);
    writer.write_header()?;

    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let camera = Vec3::new(0., 0., 0.);

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f64 / width as f64;
            let v = j as f64 / height as f64;
            writer.write(&ray_color(&Ray::new(
                        camera.clone(),
                        &lower_left_corner + u * &horizontal + v * &vertical)))?;
        }
    }

    Ok(())
}
