use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::{
    render::{Camera, Canvas},
    Scene,
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
