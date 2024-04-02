use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use ndarray::{s, Array, Axis};
use ort::{ExecutionProvider, Session};
use serde::{Deserialize, Serialize};

pub struct YoloModel {
    model: Session,
    accelerator_availability: AcceleratorAvailability,
    input_size: (usize, usize),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Detection {
    pub class: usize,
    pub score: f32,
    pub bbox: BBox,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// Availability of different accelerators
#[derive(Debug, Serialize)]
pub struct AcceleratorAvailability {
    pub coreml: bool,
    pub tensor_rt: bool,
    pub cuda: bool,
    pub direct_ml: bool,
}

impl YoloModel {
    pub fn new_from_file(
        model_path: &str,
        input_size: (usize, usize),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Loading model");

        let coreml = ort::CoreMLExecutionProvider::default()
            .with_ane_only()
            .with_subgraphs();
        println!("CoreML available: {:?}", coreml.is_available().unwrap());

        let tensor_rt = ort::TensorRTExecutionProvider::default();
        println!(
            "TensorRT available: {:?}",
            tensor_rt.is_available().unwrap()
        );

        let cuda = ort::CUDAExecutionProvider::default();
        println!("CUDA available: {:?}", cuda.is_available().unwrap());

        let direct_ml = ort::DirectMLExecutionProvider::default();
        println!(
            "DirectML available: {:?}",
            direct_ml.is_available().unwrap()
        );

        let accelerator_availability = AcceleratorAvailability {
            coreml: coreml.is_available().unwrap(),
            tensor_rt: tensor_rt.is_available().unwrap(),
            cuda: cuda.is_available().unwrap(),
            direct_ml: direct_ml.is_available().unwrap(),
        };

        let model = Session::builder()?
            .with_execution_providers(vec![
                coreml.build(),
                tensor_rt.build(),
                cuda.build(),
                direct_ml.build(),
            ])?
            .commit_from_file(model_path)?;

        println!("Model loaded");

        Ok(Self {
            model,
            input_size,
            accelerator_availability,
        })
    }

    pub fn accelerator_availability(&self) -> &AcceleratorAvailability {
        &self.accelerator_availability
    }

    pub fn detect(
        &self,
        original_img: &DynamicImage,
        conf_threshold: Option<f32>,
        nms_threshold: Option<f32>,
    ) -> Result<Vec<Detection>, Box<dyn std::error::Error>> {
        let conf_threshold = conf_threshold.unwrap_or(0.1);
        println!("Confidence threshold: {:?}", conf_threshold);
        let nms_threshold = nms_threshold.unwrap_or(0.45);
        println!("NMS threshold: {:?}", nms_threshold);

        let target_size = self.input_size.0 as u32;

        let start = std::time::Instant::now();

        let (img_width, img_height) = (original_img.width(), original_img.height());
        let img = original_img.resize_exact(target_size, target_size, FilterType::CatmullRom);
        let mut input = Array::zeros((1, 3, target_size as usize, target_size as usize));
        for pixel in img.pixels() {
            let x = pixel.0 as _;
            let y = pixel.1 as _;
            let [r, g, b, _] = pixel.2 .0;
            input[[0, 0, y, x]] = (r as f32) / 255.;
            input[[0, 1, y, x]] = (g as f32) / 255.;
            input[[0, 2, y, x]] = (b as f32) / 255.;
        }

        let outputs = self.model.run(ort::inputs!["images" => input]?)?;

        // Postprocessing
        let output = outputs["output"]
            .try_extract_tensor::<f32>()
            .unwrap()
            .view()
            .t()
            .into_owned();

        let mut boxes = Vec::new();
        let output = output.slice(s![.., .., 0]);
        for row in output.axis_iter(Axis(1)) {
            let row: Vec<_> = row.iter().copied().collect();

            let (class_id, _prob) = row
                .iter()
                // skip bounding box coordinates
                .skip(5)
                .enumerate()
                .map(|(index, value)| (index, *value))
                .reduce(|accum, row| if row.1 > accum.1 { row } else { accum })
                .unwrap();

            if row[4] < conf_threshold {
                continue;
            }

            let label = class_id;
            let xc = row[0] / target_size as f32 * (img_width as f32);
            let xc = xc.max(0.).min(img_width as f32);
            let yc = row[1] / target_size as f32 * (img_height as f32);
            let yc = yc.max(0.).min(img_height as f32);
            let w = row[2] / target_size as f32 * (img_width as f32);
            let w = w.max(0.).min(img_width as f32);
            let h = row[3] / target_size as f32 * (img_height as f32);
            let h = h.max(0.).min(img_height as f32);

            boxes.push(Detection {
                class: label,
                score: row[4],
                bbox: BBox {
                    x: xc - w / 2.0,
                    y: yc - h / 2.0,
                    w,
                    h,
                },
            });
        }

        // Non-maximum suppression
        boxes = non_max_suppression(boxes, nms_threshold);

        println!("Boxes: {:?}", boxes);

        println!("{:?}", start.elapsed());

        Ok(boxes)
    }
}

impl BBox {
    /// Calculate the intersection over union (IoU) of two bounding boxes
    pub fn iou(&self, other: &BBox) -> f32 {
        let x1 = self.x;
        let y1 = self.y;
        let w1 = self.w;
        let h1 = self.h;
        let x2 = other.x;
        let y2 = other.y;
        let w2 = other.w;
        let h2 = other.h;

        let x_a = x1.max(x2);
        let y_a = y1.max(y2);
        let x_b = (x1 + w1).min(x2 + w2);
        let y_b = (y1 + h1).min(y2 + h2);

        let inter_area = (x_b - x_a).max(0.) * (y_b - y_a).max(0.);
        let box_aarea = w1 * h1;
        let box_barea = w2 * h2;

        inter_area / (box_aarea + box_barea - inter_area)
    }
}

/// Non-Maximum Suppression
fn non_max_suppression(detections: Vec<Detection>, nms_threshold: f32) -> Vec<Detection> {
    let mut suppressed_detections: Vec<Detection> = vec![];
    let mut sorted_detections: Vec<Detection> = detections;

    sorted_detections.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
    sorted_detections.reverse();

    for i in 0..sorted_detections.len() {
        let mut keep = true;
        for j in 0..i {
            let iou = sorted_detections[i].bbox.iou(&sorted_detections[j].bbox);
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

#[cfg(test)]
mod test {

    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_model() {
        let model = YoloModel::new_from_file("../md_v5a.0.0-dynamic.onnx", (640, 640)).unwrap();

        let detections = model
            .detect(
                &image::open("./tests/fixtures/dataset/IMG_0089_peccary.JPG").unwrap(),
                Some(0.001),
                Some(0.45),
            )
            .unwrap();

        println!("{:?}", detections);

        println!("Detections: {:?}", detections.len());
    }
}
