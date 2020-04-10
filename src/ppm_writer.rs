use std::io::{BufWriter, Write};

use crate::types::Vec3;

pub struct PPMWriter<W: Write> {
    writer: BufWriter<W>,
    width: usize,
    height: usize,
}

impl<W: Write> PPMWriter<W> {
    pub fn new(writer: W, width: usize, height: usize) -> Self {
        PPMWriter { writer: BufWriter::new(writer), width, height }
    }

    pub fn write_header(&mut self) -> Result<usize, std::io::Error> {
        self.writer.write(&format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())
    }

    pub fn write(&mut self, v: &Vec3) -> Result<usize, std::io::Error> {
        self.writer.write(
            &format!(
                "{} {} {}\n",
                (255.999 * v.x).floor(),
                (255.999 * v.y).floor(),
                (255.999 * v.z).floor())
            .as_bytes())
    }
}
