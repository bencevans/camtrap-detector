use crate::{YoloDetection, YoloImageDetections};
use opencv::{
    core::{Scalar, Vector, CV_32F},
    dnn::{read_net_from_onnx, read_net_from_onnx_buffer},
    prelude::{Mat, MatTraitConst, NetTrait, NetTraitConst},
    Error,
};

pub struct YoloModel {
    net: opencv::dnn::Net,
    input_size: opencv::core::Size_<i32>,
}

impl YoloModel {
    /// Create a new YoloModel from an ONNX file.
    pub fn new_from_file(model_path: &str, input_size: (i32, i32)) -> Result<Self, Error> {
        let mut network = read_net_from_onnx(model_path)?;

        let cuda_count = opencv::core::get_cuda_enabled_device_count()?;
        println!("CUDA enabled device count: {}", cuda_count);

        if cuda_count > 0 {
            network.set_preferable_backend(opencv::dnn::DNN_BACKEND_CUDA)?;
            network.set_preferable_target(opencv::dnn::DNN_TARGET_CUDA)?;
        }

        Ok(Self {
            net: network,
            input_size: opencv::core::Size_::new(input_size.0, input_size.1),
        })
    }

    /// Create a new YoloModel from an ONNX buffer.
    pub fn new_from_buffer(buffer: &Vector<u8>, input_size: (i32, i32)) -> Result<Self, Error> {
        let mut network = read_net_from_onnx_buffer(buffer)?;

        let cuda_count = opencv::core::get_cuda_enabled_device_count()?;
        println!("CUDA enabled device count: {}", cuda_count);

        if cuda_count > 0 {
            network.set_preferable_backend(opencv::dnn::DNN_BACKEND_CUDA)?;
            network.set_preferable_target(opencv::dnn::DNN_TARGET_CUDA)?;
        }

        Ok(Self {
            net: network,
            input_size: opencv::core::Size_::new(input_size.0, input_size.1),
        })
    }

    fn load_image(&self, image_path: &str) -> Result<Mat, Error> {
        opencv::imgcodecs::imread(image_path, opencv::imgcodecs::IMREAD_COLOR)
    }

    /// Load an image from a file to OpenCV Mat.
    fn image_to_blob(&mut self, image_mat: &Mat) -> Result<Mat, Error> {
        opencv::dnn::blob_from_image(
            image_mat,
            1.0 / 255.0,
            opencv::core::Size_ {
                width: self.input_size.width,
                height: self.input_size.height,
            },
            Scalar::new(0f64, 0f64, 0f64, 0f64),
            true,
            false,
            CV_32F,
        )
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

            x_min /= 640.0;
            y_min /= 640.0;
            let mut width = *w / 640.0;
            let mut height = *h / 640.0;

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

    fn filter_confidence(&self, detections: Vec<YoloDetection>, min_confidence: f32) -> Vec<YoloDetection> {
        detections
            .into_iter()
            .filter(|dsetection| dsetection.confidence >= min_confidence)
            .collect()
    }

    fn iou(&self, a: &YoloDetection, b: &YoloDetection) -> f32 {
        let x1 = a.x.max(b.x);
        let y1 = a.y.max(b.y);
        let x2 = a.x + a.width.min(b.width);
        let y2 = a.y + a.height.min(b.height);

        let intersection = (x2 - x1).max(0.0) * (y2 - y1).max(0.0);
        let area_a = a.width * a.height;
        let area_b = b.width * b.height;

        intersection / (area_a + area_b - intersection)
    }

    fn non_max_suppression(
        &self,
        detections: Vec<YoloDetection>,
        nms_threshold: f32,
    ) -> Vec<YoloDetection> {
        let mut suppressed_detections: Vec<YoloDetection> = vec![];
        let mut sorted_detections: Vec<YoloDetection> = detections.to_vec();

        sorted_detections.sort_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());

        for i in 0..sorted_detections.len() {
            let mut keep = true;
            for j in 0..i {
                let iou = self.iou(&sorted_detections[i], &sorted_detections[j]);
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

    /// Run the model on an image and return the detections.
    pub fn detect(
        &mut self,
        image_path: &str,
        minimum_confidence: f32,
        nms_threshold: f32,
    ) -> Result<YoloImageDetections, Error> {
        // Load the image as a Mat.
        let image = self.load_image(image_path)?;
        let image_blob = self.image_to_blob(&image)?;

        // Run the model on the image.
        let result = self.forward(&image_blob)?;

        // Convert the result to a Vec of Detections.
        let detections = self.convert_to_detections(&result)?;

        // Filter the detections by confidence.
        let detections = self.filter_confidence(detections, minimum_confidence);

        // Non-maximum suppression.
        let detections = self.non_max_suppression(detections, nms_threshold);

        Ok(YoloImageDetections {
            file: image_path.to_string(),
            image_width: image.cols() as u32,
            image_height: image.rows() as u32,
            detections,
        })
    }
}
