use super::{YoloDetection, YoloImageDetections};
use image::{buffer::ConvertBuffer, GenericImageView, ImageBuffer, RgbImage, RgbaImage};
use ndarray::CowArray;
use ort::{ExecutionProvider, Session};

/// A YOLO model.
pub struct YoloModel {
    session: Session,
    input_size: (u32, u32),
}

impl YoloModel {
    /// Load a YOLO model from a file.
    pub fn new_from_file(
        path: &str,
        input_size: (u32, u32),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let coreml = ort::CoreMLExecutionProvider::default();
        if !coreml.is_available().unwrap() {
            eprintln!("Please compile ONNX Runtime with CoreML!");
        } else {
            println!("CoreML is available!");
        }

        let session = ort::Session::builder()?
            .with_execution_providers([
                // // // Prefer TensorRT over CUDA.
                // ort::TensorRTExecutionProvider::default().build(),
                // ort::CUDAExecutionProvider::default().build(),
                // // Use DirectML on Windows if NVIDIA EPs are not available
                // ort::DirectMLExecutionProvider::default().build(),
                // ort::CoreMLExecutionProvider::default()
                //     // Or use ANE on Apple platforms
                //     .with_subgraphs()
                //     // only use the ANE as the CoreML CPU implementation is super slow for this model
                //     // .with_ane_only()
                //     .build(),
            ])
            .unwrap()
            // .with_optimization_level(GraphOptimizationLevel::Level3)?
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

        // Load the image
        let (original_image, original_image_width, original_image_height) = {
            let time = std::time::Instant::now();
            println!("Loading image...");
            let original_image = image::open(image_path).unwrap();
            let original_image_width = original_image.width();
            let original_image_height = original_image.height();

            let image = original_image;
            println!("Loaded image: {:?}", time.elapsed());
            (image, original_image_width, original_image_height)
        };

        let image = {
            // 1. Letterbox the image

            let mut letterboxed: ImageBuffer<image::Rgb<f32>, Vec<f32>> =
                image::ImageBuffer::new(self.input_size.0, self.input_size.1);

            let background = image::Rgb([0.015686275, 0.015686275, 0.015686275]); // Convert background color values to f32
            for pixel in letterboxed.pixels_mut() {
                *pixel = background;
            }

            let width_w_ratio = self.input_size.0 as f32 / original_image_width as f32;
            let height_w_ratio = self.input_size.1 as f32 / original_image_height as f32;

            let ratio = width_w_ratio.min(height_w_ratio);
            let new_width = (original_image_width as f32 * ratio) as u32;
            let new_height = (original_image_height as f32 * ratio) as u32;

            let image_resized =
                original_image.resize(new_width, new_height, image::imageops::FilterType::Nearest);

            // Swap R and B channels
            let image_resized: RgbImage =
                ImageBuffer::from_fn(image_resized.width(), image_resized.height(), |x, y| {
                    let pixel = image_resized.get_pixel(x, y);
                    image::Rgb([pixel[2], pixel[1], pixel[0]])
                });

            let x = (self.input_size.0 - image_resized.width()) / 2;
            let y = (self.input_size.1 - image_resized.height()) / 2;

            let image_resized_f32: ImageBuffer<image::Rgb<f32>, Vec<_>> = image_resized.convert();
            image::imageops::overlay(&mut letterboxed, &image_resized_f32, x.into(), y.into());

            let lettboxed_to_u8: RgbaImage = letterboxed.convert();
            lettboxed_to_u8.save("letterboxed.jpg").unwrap();

            // Convert the image to a tensor
            let t = CowArray::from(ndarray::Array::from_shape_vec(
                (1, 3, self.input_size.0 as usize, self.input_size.1 as usize),
                letterboxed.as_raw().to_vec(),
            )?)
            .to_owned();

            println!("input shape = {:?}", t.shape());

            // Change the image to be BGR instead of RGB. It has the shape [1, 3, 640, 640] so we need to reverese the channels in the second axis
            // let mut t = t;
            // t.raw_view_mut().swap_axes(1, 2);

            println!("input = {:?}", t);

            t
        };

        let time = std::time::Instant::now();
        println!("Running model...");
        let binding = self.session.run(ort::inputs![image].unwrap()).unwrap();
        let binding = binding[0].extract_tensor().unwrap();
        let outputs = &binding.view();

        println!("Ran model: {:?}", time.elapsed());

        let time = std::time::Instant::now();
        println!("Processing model output...");

        let rows = *outputs.shape().get(1).unwrap();
        println!("rows: {:?}", rows);
        let mut detections = Vec::<YoloDetection>::with_capacity(rows);

        let mut overall_max_confidence: f32 = 0.0;

        for row in 0..rows {
            let cx: &f32 = &outputs[[0, row, 0]];
            let cy: &f32 = &outputs[[0, row, 1]];
            let w: &f32 = &outputs[[0, row, 2]];
            let h: &f32 = &outputs[[0, row, 3]];
            let sc: &f32 = &outputs[[0, row, 4]];

            // println!("cx: {:?}, cy: {:?}, w: {:?}, h: {:?}, sc: {:?}", cx, cy, w, h, sc);

            overall_max_confidence = overall_max_confidence.max(*sc);

            let mut x_min = *cx - *w / 2.0;
            let mut y_min = *cy - *h / 2.0;

            x_min /= self.input_size.0 as f32;
            y_min /= self.input_size.1 as f32;
            let mut width = *w / self.input_size.0 as f32;
            let mut height = *h / self.input_size.1 as f32;

            x_min = x_min.max(0.0).min(1_f32);
            y_min = y_min.max(0.0).min(1_f32);
            width = width.max(0.0).min(1_f32);
            height = height.max(0.0).min(1_f32);

            let mat_size = outputs.shape();
            let classes = *mat_size.get(2).unwrap() - 5;
            let mut classes_confidences = vec![];

            for j in 5..5 + classes {
                let confidence: &f32 = &outputs[[0, row, j]];
                classes_confidences.push(confidence);
            }

            let mut max_index = 0;
            let mut max_confidence = 0.0;
            for (index, confidence) in classes_confidences.iter().enumerate() {
                if *confidence > &max_confidence {
                    max_index = index;
                    max_confidence = **confidence;
                }
            }

            detections.push(YoloDetection {
                x: x_min,
                y: y_min,
                width,
                height,
                class_index: max_index as u32,
                confidence: *sc,
            })
        }

        println!("Overall max confidence: {:?}", overall_max_confidence);

        println!("Processed model output: {:?}", time.elapsed());
        println!("Detections: {:?}", detections.len());

        let time = std::time::Instant::now();
        println!("Filtering detections...");
        let detections = filter_confidence(detections, confidence_threshold);
        println!(
            "Filtered detections: {:?} {}",
            time.elapsed(),
            detections.len()
        );

        let time = std::time::Instant::now();
        println!("Non-maximum suppression...");
        let detections = non_max_suppression(detections, iou_threshold);
        println!(
            "Non-maximum suppression: {:?} {}",
            time.elapsed(),
            detections.len()
        );

        let detections = YoloImageDetections {
            file: image_path.to_string(),
            image_width: original_image_width,
            image_height: original_image_height,
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

#[cfg(test)]
mod test {
    use crate::yolov5::helpers::render_detections;

    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_model() {
        let mut model = YoloModel::new_from_file("../md_v5a.0.0-640x640.onnx", (640, 640)).unwrap();

        let detections = model
            .detect("./tests/fixtures/dataset/IMG_0089_peccary.JPG", 0.001, 0.45)
            .unwrap();

        println!("{:?}", detections);

        println!("Detections: {:?}", detections.detections.len());

        render_detections(
            "./tests/fixtures/dataset/IMG_0089_peccary.JPG",
            &detections,
            "tmp.jpg",
        )
        .unwrap();
    }
}
