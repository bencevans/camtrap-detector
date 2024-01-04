use std::sync::Arc;

use image::{Rgb, Rgb32FImage, RgbImage};
use ndarray::CowArray;
use ort::{
    CUDAExecutionProviderCuDNNConvAlgoSearch, ExecutionProvider, GraphOptimizationLevel, Session,
    SessionBuilder, Value,
};

use super::YoloImageDetections;

/// A YOLO model.
pub struct YoloModel {
    session: Session,
    // environment: Arc<Environment>,
}

impl YoloModel {
    /// Load a YOLO model from a file.
    pub fn new_from_file(
        path: &str,
        input_size: (usize, usize),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let environment = ort::init()
            .with_execution_providers([
                ort::CUDAExecutionProvider::default().build(),
                ort::CoreMLExecutionProvider::default().build(),
            ])
            .commit()?;

        let session = ort::Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .with_model_from_file(path)?;

        Ok(Self { session })
    }

    /// Detect objects in an image.
    pub fn detect(
        &mut self,
        image_path: &str,
        confidence_threshold: f32,
        iou_threshold: f32,
    ) -> Result<YoloImageDetections, Box<dyn std::error::Error>> {
        println!("Warning: YoloModel::detect is not implemented");
        let image = image::open(image_path).unwrap();

        let x: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::imageops::resize(&image, 640, 640, image::imageops::FilterType::Nearest);
        let mut output: image::ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::new(640, 640);
        for (output, chunk) in { output.chunks_exact_mut(3).zip(x.chunks_exact(4)) } {
            // ... and copy each of them to output, leaving out the A byte
            output.copy_from_slice(&chunk[0..3]);
        }
        // Convert x into ndarray::ArrayBase<ndarray::CowRepr<'_, f32>,

        let array: ndarray::ArrayBase<ndarray::CowRepr<'_, f32>, ndarray::Dim<ndarray::IxDynImpl>> =
            CowArray::from(ndarray::Array::from_shape_vec(
                (1, 3, 640, 640),
                output.as_raw().iter().map(|x| *x as f32).collect(),
            )?)
            .into_dyn();

        // convert image into a tensor
        // let array = ndarray::Array::from_shape_vec(
        //     (1, 3, 640, 640),
        //     vec![0.0; 1 * 3 * 640 * 640],
        // )?;
        // let tensor = ndarray_tensor(&array, self.environment.clone())?;

        // let array: ndarray::ArrayBase<ndarray::CowRepr<'_, f32>, ndarray::Dim<ndarray::IxDynImpl>> =
        //     CowArray::from(ndarray::Array::from_shape_vec(
        //         (1, 3, 640, 640),
        //         vec![0.0; 1 * 3 * 640 * 640],
        //     )?)
        //     .into_dyn();

        let inputs = ort::inputs![
            &array, // Pass the CowRepr array reference here
        ].unwrap();
        
        let outputs = &self.session.run(inputs).unwrap()[0];
        let output = outputs.extract_tensor::<f32>()?;

        // outputs[0];

        println!("outputs: {:?}", output);

        let detections = YoloImageDetections {
            file: image_path.to_string(),
            image_width: image.width(),
            image_height: image.height(),
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
        let mut model = YoloModel::new_from_file("../md_v5a.0.0-dynamic.onnx", (640, 640)).unwrap();

        let detections = model
            .detect("./tests/fixtures/dataset/IMG_0089_peccary.JPG", 0.1, 0.45)
            .unwrap();

        println!("{:?}", detections);
    }
}
