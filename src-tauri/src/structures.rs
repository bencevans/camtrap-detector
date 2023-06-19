use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct CamTrapImageDetections {
    pub file: String,
    pub error: Option<String>,

    pub image_width: Option<u32>,
    pub image_height: Option<u32>,

    pub detections: Vec<CamTrapDetection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamTrapDetection {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub class_index: u32,
    pub confidence: f32,
}

impl From<super::opencv_yolov5::YoloDetection> for CamTrapDetection {
    fn from(yolo: super::opencv_yolov5::YoloDetection) -> Self {
        Self {
            x: yolo.x,
            y: yolo.y,
            width: yolo.width,
            height: yolo.height,
            class_index: yolo.class_index,
            confidence: yolo.confidence,
        }
    }
}

impl From<super::opencv_yolov5::YoloImageDetections> for CamTrapImageDetections {
    fn from(yolo: super::opencv_yolov5::YoloImageDetections) -> Self {
        CamTrapImageDetections {
            file: yolo.file,
            error: None,
            image_width: Some(yolo.image_width),
            image_height: Some(yolo.image_height),
            detections: yolo.detections.into_iter().map(|d| d.into()).collect(),
        }
    }
}
