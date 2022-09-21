use serde::{Deserialize, Serialize};

use crate::{megadetector::CATEGORIES, structures};

#[derive(Serialize, Deserialize, Debug)]
pub struct CamTrapJSONContainer {
    pub images: Vec<CamTrapJSONImageDetections>,
    pub categories: Vec<CamTrapJSONCategory>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CamTrapJSONCategory {
    pub name: String,
    pub id: usize,
}

impl CamTrapJSONContainer {
    pub fn new(images: Vec<CamTrapJSONImageDetections>) -> Self {
        CamTrapJSONContainer {
            images,
            categories: CATEGORIES
                .iter()
                .enumerate()
                .map(|(i, c)| CamTrapJSONCategory {
                    name: c.to_string(),
                    id: i,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamTrapJSONImageDetections {
    pub file: String,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub category: u32,
    pub confidence: f32,
}

impl From<structures::CamTrapDetection> for CamTrapJSONDetection {
    fn from(yolo: structures::CamTrapDetection) -> Self {
        Self {
            x: yolo.x,
            y: yolo.y,
            width: yolo.width,
            height: yolo.height,
            category: yolo.class_index + 1,
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
