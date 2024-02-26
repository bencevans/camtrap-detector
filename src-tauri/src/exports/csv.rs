use crate::megadetector::CATEGORIES;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamTrapCSVDetection {
    /// File path
    pub file: String,

    /// Error message if any
    pub error: Option<String>,

    /// Image width in pixels
    pub image_width: Option<u32>,

    /// Image height in pixels
    pub image_height: Option<u32>,

    /// X coordinate of the top-left corner of the detection
    pub x: Option<u32>,

    /// Y coordinate of the top-left corner of the detection
    pub y: Option<u32>,

    /// Width of the detection in pixels
    pub width: Option<u32>,

    /// Height of the detection in pixels
    pub height: Option<u32>,

    /// Category of the detection
    pub category: Option<String>,

    /// Confidence of the detection
    pub confidence: Option<f32>,
}

impl CamTrapCSVDetection {
    /// Create a new error detection
    pub fn new_error(file: String, error: String) -> Self {
        Self {
            file,
            error: Some(error),
            image_width: None,
            image_height: None,
            x: None,
            y: None,
            width: None,
            height: None,
            category: None,
            confidence: None,
        }
    }

    /// Create a new empty detection
    pub fn new_empty(file: String) -> Self {
        Self {
            file,
            error: None,
            image_width: None,
            image_height: None,
            x: None,
            y: None,
            width: None,
            height: None,
            category: Some(String::from("Empty")),
            confidence: None,
        }
    }

    /// Create a new detection
    pub fn new_detection(
        file: String,
        image_width: u32,
        image_height: u32,
        detection: &crate::structures::CamTrapDetection,
    ) -> Self {
        Self {
            file,
            error: None,
            image_width: Some(image_width),
            image_height: Some(image_height),
            x: Some(detection.x as u32),
            y: Some(detection.y as u32),
            width: Some(detection.width as u32),
            height: Some(detection.height as u32),
            category: Some(
                CATEGORIES
                    .get(detection.class_index as usize + 1)
                    .unwrap()
                    .to_string(),
            ),
            confidence: Some(detection.confidence),
        }
    }
}
