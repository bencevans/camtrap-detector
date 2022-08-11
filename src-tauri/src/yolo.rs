use opencv::{
  core::{Scalar, CV_32F, Vector},
  dnn::{read_net_from_onnx, read_net_from_onnx_str, read_net_from_onnx_buffer},
  prelude::{Mat, MatTraitConst, NetTrait, NetTraitConst},
  Error,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxDetection {
  pub x: f64,
  pub y: f64,
  pub w: f64,
  pub h: f64,
  pub confidence: f64,
  pub class_id: f64,
}



pub fn load_model_from_bytes(vector: &Vector<u8>) -> Result<opencv::dnn::Net, Error> {
  read_net_from_onnx_buffer(vector)
}

pub fn load_model(model_path: &str) -> Result<opencv::dnn::Net, Error> {
  read_net_from_onnx(model_path)
}

pub fn infer(
  model: &mut opencv::dnn::Net,
  image_path: &str,
  min_confidence: &f32,
  nms_threshold: f64,
) -> Result<Vec<BoxDetection>, Error> {
  let image = opencv::imgcodecs::imread(image_path, opencv::imgcodecs::IMREAD_COLOR)?;
  let image_width = image.cols();
  let image_height = image.rows();

  let blob = opencv::dnn::blob_from_image(
      &image,
      1.0 / 255.0,
      opencv::core::Size_ {
          width: 640,
          height: 640,
      },
      Scalar::new(0f64, 0f64, 0f64, 0f64),
      true,
      false,
      CV_32F,
  )?;

  let mut output_tensor_blobs: opencv::core::Vector<Mat> = opencv::core::Vector::default();

  model.set_input(&blob, "", 1.0, Scalar::default())?;
  model.forward(
      &mut output_tensor_blobs,
      &model.get_unconnected_out_layers_names()?,
  )?;

  let outputs = output_tensor_blobs.get(0)?;

  let rows = *outputs.mat_size().get(1).unwrap();

  let mut detections: Vec<BoxDetection> = vec![];

  for i in 0..rows {
      let cx: &f32 = outputs.at_3d(0, i, 0)?;
      let cy: &f32 = outputs.at_3d(0, i, 1)?;
      let w: &f32 = outputs.at_3d(0, i, 2)?;
      let h: &f32 = outputs.at_3d(0, i, 3)?;
      let sc: &f32 = outputs.at_3d(0, i, 4)?;

      let mut x_min = *cx - *w / 2.0;
      let mut y_min = *cy - *h / 2.0;

      x_min /= 640.0;
      y_min /= 640.0;
      let mut width = *w / 640.0;
      let mut height = *h / 640.0;

      x_min *= image_width as f32;
      y_min *= image_height as f32;
      width *= image_width as f32;
      height *= image_height as f32;

      // ensure within image bounds
      x_min = x_min.max(0.0).min(image_width as f32);
      y_min = y_min.max(0.0).min(image_height as f32);
      width = width.max(0.0).min(image_width as f32);
      height = height.max(0.0).min(image_height as f32);

      if sc < min_confidence {
          continue;
      }

      let detection = BoxDetection {
          x: x_min as f64,
          y: y_min as f64,
          w: width as f64,
          h: height as f64,
          confidence: *sc as f64,
          class_id: 0.0,
      };

      detections.push(detection);
  }

  // Non Max Suppression
  let mut nms_detections: Vec<BoxDetection> = vec![];

  for i in 0..detections.len() {
      let mut keep = true;
      for j in 0..nms_detections.len() {
          if i != j && keep {
              let i_x = detections[i].x;
              let i_y = detections[i].y;
              let i_w = detections[i].w;
              let i_h = detections[i].h;
              let _i_conf = detections[i].confidence;
              let j_x = detections[j].x;
              let j_y = detections[j].y;
              let j_w = detections[j].w;
              let j_h = detections[j].h;
              let _j_conf = detections[j].confidence;
              let i_area = i_w * i_h;
              let j_area = j_w * j_h;
              let area = i_area + j_area;
              let union = (i_x - j_x).powi(2) + (i_y - j_y).powi(2);
              let iou = area - union / area;
              if iou > nms_threshold {
                  keep = false;
              }
          }
      }
      if keep {
          nms_detections.push(detections[i].clone());
      }
  }

  Ok(nms_detections)
}
