use image::Rgb;
use ndarray::CowArray;
use ort::{ExecutionProvider, GraphOptimizationLevel, Session};

use super::{YoloDetection, YoloImageDetections};

/// A YOLO model.
pub struct YoloModel {
    session: Session,
    // environment: Arc<Environment>,
    input_size: (u32, u32),
}

impl YoloModel {
    /// Load a YOLO model from a file.
    pub fn new_from_file(
        path: &str,
        input_size: (u32, u32),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // tracing_subscriber::fmt::init();
        // let dylib_path = crate::internal::find_onnxruntime_dylib()?; // /etc/.../libonnxruntime.so
        // println!("dylib_path: {:?}", dylib_path);

        // ort::init_from(dylib_path).commit()?;

        let coreml = ort::CoreMLExecutionProvider::default();
        if !coreml.is_available().unwrap() {
            eprintln!("Please compile ONNX Runtime with CoreML!");
        } else {
            println!("CoreML is available!");
        }

        let session = ort::Session::builder()?
            .with_execution_providers([
                // // Prefer TensorRT over CUDA.
                ort::TensorRTExecutionProvider::default().build(),
                ort::CUDAExecutionProvider::default().build(),
                // Use DirectML on Windows if NVIDIA EPs are not available
                ort::DirectMLExecutionProvider::default().build(),
                ort::CoreMLExecutionProvider::default()
                    // Or use ANE on Apple platforms
                    .with_subgraphs()
                    // only use the ANE as the CoreML CPU implementation is super slow for this model
                    // .with_ane_only()
                    .build(),
            ])
            .unwrap()
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            // .with_intra_threads(4)?
            .with_model_from_file(path)?;

        println!("{:?}", session.allocator());

        Ok(Self {
            session,
            input_size,
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

        let time = std::time::Instant::now();
        println!("Loading image...");
        let image = image::open(image_path).unwrap();
        println!("Loaded image: {:?}", time.elapsed());

        // TODO: Letterboxing

        let time = std::time::Instant::now();
        println!("Resizing image...");
        let x: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::imageops::resize(
            &image,
            self.input_size.0,
            self.input_size.1,
            image::imageops::FilterType::Nearest,
        );
        println!("Resized image: {:?}", time.elapsed());

        let time = std::time::Instant::now();
        println!("Converting image...");
        let mut output: image::ImageBuffer<Rgb<u8>, Vec<_>> =
            image::ImageBuffer::new(self.input_size.0, self.input_size.1);
        for (output, chunk) in output.chunks_exact_mut(3).zip(x.chunks_exact(4)) {
            // ... and copy each of them to output, leaving out the A byte
            output.copy_from_slice(&chunk[0..3]);
        }
        println!("Converted image: {:?}", time.elapsed());
        // Convert x into ndarray::ArrayBase<ndarray::CowRepr<'_, f32>,

        let time = std::time::Instant::now();
        println!("Converting image to tensor...");
        let array: ndarray::ArrayBase<ndarray::CowRepr<'_, f32>, ndarray::Dim<ndarray::IxDynImpl>> =
            CowArray::from(ndarray::Array::from_shape_vec(
                (1, 3, self.input_size.0 as usize, self.input_size.1 as usize),
                output.as_raw().iter().map(|x| *x as f32).collect(),
            )?)
            .into_dyn();

        // Pretend it's a batch of 12 by repeating the first image 12 times
        let array = array
            .broadcast((1, 3, self.input_size.0 as usize, self.input_size.1 as usize))
            .unwrap()
            .to_owned();
        println!("Converted image to tensor: {:?}", time.elapsed());

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

        let time = std::time::Instant::now();
        println!("Running model...");
        let inputs = ort::inputs![
            array, // Pass the CowRepr array reference here
        ]
        .unwrap();

        let outputs = &self.session.run(inputs).unwrap()[0];
        let output = outputs.extract_tensor::<f32>()?;
        println!("Ran model: {:?}", time.elapsed());

        let time = std::time::Instant::now();
        println!("Processing output...");
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
        println!("Processed output: {:?}", time.elapsed());

        let time = std::time::Instant::now();
        println!("Filtering detections...");
        let detections = filter_confidence(detections, confidence_threshold);
        println!("Filtered detections: {:?}", time.elapsed());

        let time = std::time::Instant::now();
        println!("Non-maximum suppression...");
        let detections = non_max_suppression(detections, iou_threshold);
        println!("Non-maximum suppression: {:?}", time.elapsed());

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
