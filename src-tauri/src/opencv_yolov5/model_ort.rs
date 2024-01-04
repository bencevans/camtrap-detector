use std::sync::Arc;

use image::{Rgb, Rgb32FImage, RgbImage};
use ndarray::{Axis, CowArray};
use ort::{
    CUDAExecutionProviderCuDNNConvAlgoSearch, ExecutionProvider, GraphOptimizationLevel, Session,
    SessionBuilder, Value,
};

use super::{YoloDetection, YoloImageDetections};

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

        // TODO: Letterboxing

        let x: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::imageops::resize(&image, 640, 640, image::imageops::FilterType::Nearest);
        let mut output: image::ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::new(640, 640);
        for (output, chunk) in output.chunks_exact_mut(3).zip(x.chunks_exact(4)) {
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
        ]
        .unwrap();

        let outputs = &self.session.run(inputs).unwrap()[0];
        let output = outputs.extract_tensor::<f32>()?;
        let view = output.view();

        let mut detections = vec![];

        for i in 0..25500 {
            let x = view[[0, i, 0]];
            let y = view[[0, i, 1]];
            let width = view[[0, i, 2]];
            let height = view[[0, i, 3]];
            let class_index = view[[0, i, 4]];
            let confidence = view[[0, i, 5]];

            if confidence > confidence_threshold {
                detections.push(YoloDetection {
                    x,
                    y,
                    width,
                    height,
                    class_index: class_index as u32,
                    confidence,
                });
            }
        }

        let detections = YoloImageDetections {
            file: image_path.to_string(),
            image_width: image.width(),
            image_height: image.height(),
            detections,
        };

        Ok(detections)
    }
}

/// Calculate Intersection Over Union (IOU) between two bounding boxes.
fn iou(a: &YoloDetection, b: &YoloDetection) -> f32 {
    let area_a = a.area();
    let area_b = b.area();

    let top_left = (a.x.max(b.x), a.y.max(b.y));
    let bottom_right = (a.x + a.width.min(b.width), a.y + a.height.min(b.height));

    let intersection =
        (bottom_right.0 - top_left.0).max(0.0) * (bottom_right.1 - top_left.1).max(0.0);

    intersection / (area_a + area_b - intersection)
}

/// Non-Maximum Suppression
fn non_max_suppression(detections: Vec<YoloDetection>, nms_threshold: f32) -> Vec<YoloDetection> {
    let mut suppressed_detections: Vec<YoloDetection> = vec![];
    let mut sorted_detections: Vec<YoloDetection> = detections.to_vec();

    sorted_detections.sort_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());
    sorted_detections.reverse();

    for i in 0..sorted_detections.len() {
        let mut keep = true;
        for j in 0..i {
            let iou = iou(&sorted_detections[i], &sorted_detections[j]);
            if iou > nms_threshold {
                keep = false;
                break;
            }
        }
        if keep {
            suppressed_detections.push(sorted_detections[i].clone());
        }
    }
    suppressed_detections
}

/// Filter detections by confidence.
fn filter_confidence(detections: Vec<YoloDetection>, min_confidence: f32) -> Vec<YoloDetection> {
    detections
        .into_iter()
        .filter(|dsetection| dsetection.confidence >= min_confidence)
        .collect()
}

// fn convert_to_detections(outputs: &Mat) -> Result<Vec<YoloDetection>, Error> {
//     let rows = *outputs.mat_size().get(1).unwrap();
//     let mut detections = Vec::<YoloDetection>::with_capacity(rows as usize);

//     for row in 0..rows {
//         let cx: &f32 = outputs.at_3d(0, row, 0)?;
//         let cy: &f32 = outputs.at_3d(0, row, 1)?;
//         let w: &f32 = outputs.at_3d(0, row, 2)?;
//         let h: &f32 = outputs.at_3d(0, row, 3)?;
//         let sc: &f32 = outputs.at_3d(0, row, 4)?;

//         let mut x_min = *cx - *w / 2.0;
//         let mut y_min = *cy - *h / 2.0;

//         x_min /= self.input_size.width as f32;
//         y_min /= self.input_size.height as f32;
//         let mut width = *w / self.input_size.width as f32;
//         let mut height = *h / self.input_size.height as f32;

//         x_min = x_min.max(0.0).min(1_f32);
//         y_min = y_min.max(0.0).min(1_f32);
//         width = width.max(0.0).min(1_f32);
//         height = height.max(0.0).min(1_f32);

//         let mat_size = outputs.mat_size();
//         let classes = *mat_size.get(2).unwrap() - 5;
//         let mut classes_confidences = vec![];

//         for j in 5..5 + classes {
//             let confidence: &f32 = outputs.at_3d(0, row, j)?;
//             classes_confidences.push(confidence);
//         }

//         let mut max_index = 0;
//         let mut max_confidence = 0.0;
//         for (index, confidence) in classes_confidences.iter().enumerate() {
//             if *confidence > &max_confidence {
//                 max_index = index;
//                 max_confidence = **confidence;
//             }
//         }

//         detections.push(YoloDetection {
//             x: x_min,
//             y: y_min,
//             width,
//             height,
//             class_index: max_index as u32,
//             confidence: *sc,
//         })
//     }

//     Ok(detections)
// }

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
