use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

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

    pub fn save_json_relative(&self, base_path: String, file_path: &Path) {
        let rel_self = &mut (*self).clone();
        for image in rel_self.images.iter_mut() {
            image.file = image.file.replace(&base_path, "");
        }
        rel_self.save_json(file_path);
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
