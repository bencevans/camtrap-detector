mod detections;
pub mod helpers;
mod model;

pub use detections::YoloDetection;
pub use detections::YoloImageDetections;
pub use model::YoloModel;

#[cfg(test)]
mod tests {
    use crate::helpers::render_detections;

    use super::*;

    #[test]
    fn create_model() {
        let image_path = "/Users/ben/demo-dataset/IMG_0173_multi.JPG";

        let model = YoloModel::new_from_file(
            "/Users/ben/Projects/camtrap-detector/md_v5a.0.0.onnx",
            (640, 640),
        );
        assert!(model.is_ok());

        let mut model = model.unwrap();

        let detections = model.detect(image_path, 0.1, 0.45);

        assert!(detections.is_ok());

        let detections = detections.unwrap();

        assert_eq!(detections.detections.len(), 1);

        render_detections(image_path, &detections, "output.jpg").unwrap();
    }
}
