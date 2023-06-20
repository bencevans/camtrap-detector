use super::{YoloDetection, YoloImageDetections};
use opencv::{
    core::{copy_make_border, Scalar, Vector, BORDER_CONSTANT, CV_32F},
    dnn::{read_net_from_onnx, read_net_from_onnx_buffer},
    prelude::{Mat, MatTraitConst, NetTrait, NetTraitConst, MatTraitConstManual},
    Error,
};
use tracing::info;

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

/// Wrapper around OpenCV's DNN module for YOLOv5 inference.
pub struct YoloModel {
    net: opencv::dnn::Net,
    input_size: opencv::core::Size_<i32>,
}

impl YoloModel {
    /// Create a new YoloModel from an ONNX file.
    pub fn new_from_file(model_path: &str, input_size: (i32, i32)) -> Result<Self, Error> {
        YoloModel::new_from_network(read_net_from_onnx(model_path)?, input_size)
    }

    /// Create a new YoloModel from an ONNX buffer.
    pub fn new_from_buffer(buffer: &Vector<u8>, input_size: (i32, i32)) -> Result<Self, Error> {
        YoloModel::new_from_network(read_net_from_onnx_buffer(buffer)?, input_size)
    }

    /// Create a new YoloModel from an pre-loaded OpenCV DNN network.
    pub fn new_from_network(
        mut network: opencv::dnn::Net,
        input_size: (i32, i32),
    ) -> Result<Self, Error> {
        let cuda_count = opencv::core::get_cuda_enabled_device_count()?;
        info!("CUDA enabled device count: {}", cuda_count);

        if cuda_count > 0 {
            network.set_preferable_backend(opencv::dnn::DNN_BACKEND_CUDA)?;
            network.set_preferable_target(opencv::dnn::DNN_TARGET_CUDA)?;
        }

        Ok(Self {
            net: network,
            input_size: opencv::core::Size_::new(input_size.0, input_size.1),
        })
    }

    /// Load an OpenCV image, resize and adjust the color channels.
    fn load_image(&self, image_path: &str) -> Result<(Mat, u32, u32), Error> {
        println!("image path: {}", image_path);
        let image = opencv::imgcodecs::imread(image_path, opencv::imgcodecs::IMREAD_COLOR)?;

        println!("image size: {:?}", image.size()?);

        let mut boxed_image = Mat::default();

        copy_make_border(
            &image,
            &mut boxed_image,
            0,
            0,
            0,
            0,
            BORDER_CONSTANT,
            Scalar::new(114f64, 114f64, 114f64, 0f64),
        )?;

        let width = image.cols() as u32;
        let height = image.rows() as u32;

        // println!("scale factor: {:?}", 1.0 / 255.0);
        println!("{} {}", self.input_size.width, self.input_size.height);

        let blob = opencv::dnn::blob_from_image(
            &boxed_image,
            1.0 / 255.0,
            opencv::core::Size_ {
                width: self.input_size.width,
                height: self.input_size.height,
            },
            Scalar::new(0f64, 0f64, 0f64, 0f64),
            true,
            false,
            CV_32F,
        )?;

        Ok((blob, width, height))
    }

    /// Detect objects in an image.
    fn forward(&mut self, blob: &Mat) -> Result<Mat, Error> {
        let mut output_tensor_blobs: opencv::core::Vector<Mat> = opencv::core::Vector::default();

        self.net.set_input(&blob, "", 1.0, Scalar::default())?;
        self.net.forward(
            &mut output_tensor_blobs,
            &self.net.get_unconnected_out_layers_names()?,
        )?;

        output_tensor_blobs.get(0)
    }

    /// Convert the output of the YOLOv5 model to a vector of [YoloDetection].
    fn convert_to_detections(&self, outputs: &Mat) -> Result<Vec<YoloDetection>, Error> {
        let rows = *outputs.mat_size().get(1).unwrap();
        let mut detections = Vec::<YoloDetection>::with_capacity(rows as usize);

        for row in 0..rows {
            let cx: &f32 = outputs.at_3d(0, row, 0)?;
            let cy: &f32 = outputs.at_3d(0, row, 1)?;
            let w: &f32 = outputs.at_3d(0, row, 2)?;
            let h: &f32 = outputs.at_3d(0, row, 3)?;
            let sc: &f32 = outputs.at_3d(0, row, 4)?;

            let mut x_min = *cx - *w / 2.0;
            let mut y_min = *cy - *h / 2.0;

            x_min /= self.input_size.width as f32;
            y_min /= self.input_size.height as f32;
            let mut width = *w / self.input_size.width as f32;
            let mut height = *h / self.input_size.height as f32;

            x_min = x_min.max(0.0).min(1_f32);
            y_min = y_min.max(0.0).min(1_f32);
            width = width.max(0.0).min(1_f32);
            height = height.max(0.0).min(1_f32);

            let mat_size = outputs.mat_size();
            let classes = *mat_size.get(2).unwrap() - 5;
            let mut classes_confidences = vec![];

            for j in 5..5 + classes {
                let confidence: &f32 = outputs.at_3d(0, row, j)?;
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

        Ok(detections)
    }

    /// Run the model on an image and return the detections.
    pub fn detect(
        &mut self,
        image_path: &str,
        minimum_confidence: f32,
        nms_threshold: f32,
    ) -> Result<YoloImageDetections, Error> {
        // Load the image
        let (image, image_width, image_height) = self.load_image(image_path)?;

        // Run the model on the image.
        let result = self.forward(&image)?;

        // Convert the result to a Vec of Detections.
        let detections = self.convert_to_detections(&result)?;

        // Filter the detections by confidence.
        let detections = filter_confidence(detections, minimum_confidence);

        // Non-maximum suppression.
        let detections = non_max_suppression(detections, nms_threshold);

        Ok(YoloImageDetections {
            file: image_path.to_string(),
            image_width,
            image_height,
            detections,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iou() {
        let detection_a = YoloDetection {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
            class_index: 0,
            confidence: 0.0,
        };

        let detection_b = YoloDetection {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 0.5,
            class_index: 0,
            confidence: 0.0,
        };

        assert_eq!(iou(&detection_a, &detection_a), 1.0);
        assert_eq!(iou(&detection_a, &detection_b), 0.5);

        let detection_a = YoloDetection {
            x: 10.0,
            y: 10.0,
            width: 1.0,
            height: 1.0,
            class_index: 0,
            confidence: 0.0,
        };

        let detection_b = YoloDetection {
            x: 10.0,
            y: 10.0,
            width: 1.0,
            height: 0.5,
            class_index: 0,
            confidence: 0.0,
        };

        assert_eq!(iou(&detection_a, &detection_a), 1.0);
        assert_eq!(iou(&detection_a, &detection_b), 0.5);

        let detection_a = YoloDetection {
            x: 0.0,
            y: 0.0,
            width: 3.0,
            height: 3.0,
            class_index: 0,
            confidence: 0.0,
        };

        let detection_b = YoloDetection {
            x: 2.0,
            y: 0.0,
            width: 3.0,
            height: 3.0,
            class_index: 0,
            confidence: 0.0,
        };

        assert_eq!(iou(&detection_a, &detection_a), 1.0);
        assert_eq!(iou(&detection_a, &detection_b), 0.2);
    }
}
