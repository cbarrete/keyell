use std::io::Write;

use crate::render::{Canvas, Color};

pub struct PpmWriter<W: Write> {
    writer: W,
    width: usize,
    height: usize,
}

impl<W: Write> PpmWriter<W> {
    pub fn new(writer: W, canvas: &Canvas) -> Self {
        PpmWriter {
            writer,
            width: canvas.width,
            height: canvas.height,
        }
    }

    pub fn write_header(&mut self) -> Result<usize, std::io::Error> {
        self.writer
            .write(&format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())
    }

    pub fn write_pixel(&mut self, c: &Color) -> Result<usize, std::io::Error> {
        self.writer.write(
            &format!(
                "{} {} {}\n",
                (255.999 * c.r).floor(),
                (255.999 * c.g).floor(),
                (255.999 * c.b).floor()
            )
            .as_bytes(),
        )
    }
}
