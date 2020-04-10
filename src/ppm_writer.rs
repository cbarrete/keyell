use std::io::{BufWriter, Write};

use crate::types::{Canvas, Color};

pub struct PPMWriter<W: Write> {
    writer: BufWriter<W>,
    width: usize,
    height: usize,
}

impl<W: Write> PPMWriter<W> {
    pub fn new(writer: W, canvas: &Canvas) -> Self {
        PPMWriter {
            writer: BufWriter::new(writer),
            width: canvas.width,
            height: canvas.height,
        }
    }

    pub fn write_header(&mut self) -> Result<usize, std::io::Error> {
        self.writer.write(&format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())
    }

    pub fn write(&mut self, c: &Color) -> Result<usize, std::io::Error> {
        self.writer.write(
            &format!(
                "{} {} {}\n",
                (255.999 * c.r).floor(),
                (255.999 * c.g).floor(),
                (255.999 * c.b).floor())
            .as_bytes())
    }
}
