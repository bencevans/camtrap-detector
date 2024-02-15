use serde::{Deserialize, Serialize};

/// A structure to hold the detections found in an image
#[derive(Debug, Clone)]
pub struct CamTrapImageDetections {
    /// The file path of the image
    pub file: String,

    /// An error message if the image could not be processed
    pub error: Option<String>,

    /// The width of the image in pixels
    pub image_width: Option<u32>,

    /// The height of the image in pixels
    pub image_height: Option<u32>,

    /// The detections found in the image
    pub detections: Vec<CamTrapDetection>,
}

/// An individual detection found in an image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamTrapDetection {
    /// The x-coordinate of the top-left corner of the bounding box
    pub x: f32,

    /// The y-coordinate of the top-left corner of the bounding box
    pub y: f32,

    /// The width of the bounding box
    pub width: f32,

    /// The height of the bounding box
    pub height: f32,

    /// The index of the class detected
    pub class_index: u32,

    /// The confidence of the detection
    pub confidence: f32,
}

impl From<super::yolov5::YoloDetection> for CamTrapDetection {
    fn from(yolo: super::yolov5::YoloDetection) -> Self {
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

impl From<super::yolov5::YoloImageDetections> for CamTrapImageDetections {
    fn from(yolo: super::yolov5::YoloImageDetections) -> Self {
        CamTrapImageDetections {
            file: yolo.file,
            error: None,
            image_width: Some(yolo.image_width),
            image_height: Some(yolo.image_height),
            detections: yolo.detections.into_iter().map(|d| d.into()).collect(),
        }
    }
}
