use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoloImageDetections {
    pub file: String,

    pub image_width: u32,
    pub image_height: u32,

    pub detections: Vec<YoloDetection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoloDetection {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub class_index: u32,
    pub confidence: f32,
}
