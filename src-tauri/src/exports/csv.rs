use serde::{Deserialize, Serialize};

use crate::megadetector::CATEGORIES;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamTrapCSVDetection {
    pub file: String,
    pub error: Option<String>,

    pub image_width: Option<u32>,
    pub image_height: Option<u32>,

    pub x: Option<u32>,
    pub y: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub category: Option<String>,
    pub confidence: Option<f32>,
}

impl CamTrapCSVDetection {
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
            x: Some((detection.x * (image_width as f32)) as u32),
            y: Some((detection.y * (image_height as f32)) as u32),
            width: Some((detection.width * (image_width as f32)) as u32),
            height: Some((detection.height * (image_height as f32)) as u32),
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
