mod detections;
pub mod helpers;
mod model;
pub mod model_ort;

pub use detections::YoloDetection;
pub use detections::YoloImageDetections;
pub use model::YoloModel;

#[cfg(test)]
mod tests {
    use super::helpers::render_detections;

    use super::*;

    #[test]
    fn create_model() {
        let image_path = "tests/fixtures/dataset/IMG_0089_peccary.JPG";

        let model = YoloModel::new_from_file("../md_v5a.0.0-1280x1280.onnx", (1280, 1280));

        let mut model = model.unwrap();

        let detections = model.detect(image_path, 0.5, 0.45);

        let detections = detections.unwrap();

        assert_eq!(detections.image_width, 4608);
        assert_eq!(detections.image_height, 2560);
        assert_eq!(detections.file, image_path.to_string());

        assert_eq!(detections.detections.len(), 1);

        render_detections(image_path, &detections, "output.jpg").unwrap();
    }
}
