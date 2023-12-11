use std::sync::Arc;

use ndarray::CowArray;
use ort::{
    tensor::ndarray_tensor, Environment, ExecutionProvider, GraphOptimizationLevel, Session,
    SessionBuilder, Value,
};

use super::YoloImageDetections;

/// A YOLO model.
pub struct YoloModel {
    session: Session,
    environment: Arc<Environment>,
}

impl YoloModel {
    /// Load a YOLO model from a file.
    pub fn new_from_file(
        path: &str,
        input_size: (usize, usize),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let environment = Environment::builder()
            .with_name("CamTrap Detector")
            .with_execution_providers([
                ExecutionProvider::CUDA(Default::default()),
                ExecutionProvider::CoreML(Default::default()),
            ])
            .build()?
            .into_arc();

        let session = SessionBuilder::new(&environment)?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .with_model_from_file(path)?;

        Ok(Self {
            session,
            environment,
        })
    }

    /// Detect objects in an image.
    pub fn detect(
        &mut self,
        image_path: &str,
        confidence_threshold: f32,
        iou_threshold: f32,
    ) -> Result<YoloImageDetections, Box<dyn std::error::Error>> {
        println!("Warning: YoloModel::detect is not implemented");
        let image = image::open(image_path)?;

        let array: ndarray::ArrayBase<ndarray::CowRepr<'_, f32>, ndarray::Dim<ndarray::IxDynImpl>> =
            CowArray::from(ndarray::Array::from_shape_vec(
                (1, 3, 640, 640),
                vec![0.0; 1 * 3 * 640 * 640],
            )?)
            .into_dyn();

        let inputs = Value::from_array(
            self.session.allocator(),
            &array, // Pass the CowRepr array reference here
        )
        .unwrap();
        let outputs = self.session.run(vec![inputs]);

        let detections = YoloImageDetections {
            file: "mock".to_string(),
            image_width: 111,
            image_height: 111,
            detections: vec![],
        };

        Ok(detections)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_model() {
        let model = YoloModel::new_from_file("../md_v5a.0.0-dynamic.onnx", (640, 640)).unwrap();
    }
}
