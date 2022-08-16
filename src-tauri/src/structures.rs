use pathdiff::diff_paths;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CSVOutput {
    pub file: String,
    pub error: Option<String>,
    pub detection_category: Option<String>,
    pub detection_confidence: Option<f32>,
    pub detection_x: Option<f32>,
    pub detection_y: Option<f32>,
    pub detection_width: Option<f32>,
    pub detection_height: Option<f32>,
}

impl CSVOutput {
    pub fn new_empty(file: String) -> CSVOutput {
        CSVOutput {
            file,
            error: None,
            detection_category: Some("Empty".to_string()),
            detection_confidence: None,
            detection_x: None,
            detection_y: None,
            detection_width: None,
            detection_height: None,
        }
    }

    pub fn new_error(file: String, error: String) -> CSVOutput {
        CSVOutput {
            file,
            error: Some(error),
            detection_category: None,
            detection_confidence: None,
            detection_x: None,
            detection_y: None,
            detection_width: None,
            detection_height: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MegaDetectorBatchOutput {
    pub images: Vec<MegaDetectorFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_categories: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<HashMap<String, String>>,
}

impl MegaDetectorBatchOutput {
    pub fn save_json(&self, file_path: &Path) {
        let mut file = std::fs::File::create(file_path).unwrap();
        serde_json::to_writer_pretty(&mut file, &self).unwrap();
    }

    pub fn save_json_relative(&self, base_path: &String, file_path: &Path) {
        let rel_self = &mut (*self).clone();
        for image in rel_self.images.iter_mut() {
            image.file = diff_paths(&image.file, base_path)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        }
        rel_self.save_json(file_path);
    }

    pub fn save_csv(&self, file_path: &Path) {
        let file = std::fs::File::create(file_path).unwrap();
        let mut wtr = csv::Writer::from_writer(file);
        for image in self.images.iter() {
            if image.error.is_some() {
                wtr.serialize(CSVOutput::new_error(
                    image.file.clone(),
                    image.error.as_ref().unwrap().clone(),
                ))
                .unwrap();
            } else if image.detections.is_none()
                || (image.detections.is_some() && image.detections.as_ref().unwrap().is_empty())
            {
                wtr.serialize(CSVOutput::new_empty(image.file.clone()))
                    .unwrap();
            } else {
                for detection in image.detections.as_ref().unwrap().iter() {
                    wtr.serialize(CSVOutput {
                        file: image.file.clone(),
                        error: None,
                        detection_category: match detection.category.as_str() {
                            "1" => Some("animal".to_string()),
                            "2" => Some("human".to_string()),
                            "3" => Some("vehicle".to_string()),
                            _ => Some("unknown".to_string()),
                        },
                        detection_confidence: Some(detection.conf),
                        detection_x: Some(detection.bbox[0]),
                        detection_y: Some(detection.bbox[1]),
                        detection_width: Some(detection.bbox[2]),
                        detection_height: Some(detection.bbox[3]),
                    })
                    .unwrap();
                }
            }
        }
        wtr.flush().unwrap();
    }

    pub fn save_csv_relative(&self, base_path: &String, file_path: &Path) {
        let rel_self = &mut (*self).clone();
        for image in rel_self.images.iter_mut() {
            image.file = diff_paths(&image.file, base_path)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        }
        rel_self.save_csv(file_path);
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MegaDetectorFile {
    pub file: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub detections: Option<Vec<Detection>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Detection {
    pub category: String,
    pub conf: f32,
    pub bbox: [f32; 4],
}

#[cfg(test)]
mod tests {
    use std::path;

    use crate::structures::MegaDetectorBatchOutput;

    #[test]
    fn reading_writing_should_match() {
        const FIXTURE_PATH: &str = "./tests/fixtures/ena.md.4.1.0.json";
        let file = std::fs::read_to_string(path::Path::new(FIXTURE_PATH)).unwrap();
        let parsed = serde_json::from_str::<MegaDetectorBatchOutput>(&file).unwrap();
        let serialized = serde_json::to_string_pretty(&parsed).unwrap();
        let parsed_again = serde_json::from_str::<MegaDetectorBatchOutput>(&serialized).unwrap();

        assert_eq!(parsed, parsed_again);
    }
}
