use std::{ops::Range, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{
    render::{Camera, Canvas},
    Scene,
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
