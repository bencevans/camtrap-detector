use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MegaDetectorBatchOutput {
    images: Vec<MegaDetectorFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    detection_categories: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<HashMap<String, String>>,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MegaDetectorFile {
    file: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    detections: Option<Vec<Detection>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Detection {
    category: String,
    conf: f32,
    bbox: [f32; 4],
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