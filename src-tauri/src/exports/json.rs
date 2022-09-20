use serde::{Serialize, Deserialize};

use crate::{structures, megadetector::CATEGORIES};

#[derive(Debug, Clone)]
pub struct CamTrapJSONImageDetections {
    pub file: String,
    pub error: Option<String>,

    pub image_width: Option<u32>,
    pub image_height: Option<u32>,

    pub detections: Vec<CamTrapJSONDetection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamTrapJSONDetection {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub category: String,
    pub confidence: f32,
}

impl From<structures::CamTrapDetection> for CamTrapJSONDetection {
    fn from(yolo: structures::CamTrapDetection) -> Self {
        Self {
            x: yolo.x,
            y: yolo.y,
            width: yolo.width,
            height: yolo.height,
            category: CATEGORIES.get(yolo.class_index as usize).unwrap().to_string(),
            confidence: yolo.confidence,
        }
    }
}

impl From<structures::CamTrapImageDetections> for CamTrapJSONImageDetections {
    fn from(yolo: structures::CamTrapImageDetections) -> Self {
        CamTrapJSONImageDetections {
            file: yolo.file,
            error: None,
            image_width: yolo.image_width,
            image_height: yolo.image_height,
            detections: yolo.detections.into_iter().map(|d| d.into()).collect(),
        }
    }
}