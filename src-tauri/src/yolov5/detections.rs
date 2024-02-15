use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Representation of an image with it's detections.
pub struct YoloImageDetections {
    /// File Path.
    pub file: String,

    /// Image Width in Pixels.
    pub image_width: u32,

    // Image Height in Pixels.
    pub image_height: u32,

    /// Array of [YoloDetection]s.
    pub detections: Vec<YoloDetection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Representation of an object detection within an image.
pub struct YoloDetection {
    /// Top-Left Bounds Coordinate in X-Axis
    pub x: f32,

    // Top-Left Bounds Coordinate in Y-Axis
    pub y: f32,

    /// Width of Bounding Box
    pub width: f32,

    /// Height of Bounding Box
    pub height: f32,

    /// Class Index
    pub class_index: u32,

    /// Softmaxed Activation
    pub confidence: f32,
}

impl YoloDetection {
    /// Calculate the area of the detection.
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}
