use std::collections::HashSet;

use opencv::{
    core::{Scalar, Vector, CV_32F},
    dnn::{read_net_from_onnx, read_net_from_onnx_buffer},
    prelude::{Mat, MatTraitConst, NetTrait, NetTraitConst},
    Error,
};

use crate::structures::Detection;

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
    nms_threshold: f32,
) -> Result<Vec<Detection>, Error> {
    let image = opencv::imgcodecs::imread(image_path, opencv::imgcodecs::IMREAD_COLOR)?;

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

    let mut detections: Vec<Detection> = vec![];

    for i in 0..rows {
        let cx: &f32 = outputs.at_3d(0, i, 0)?;
        let cy: &f32 = outputs.at_3d(0, i, 1)?;
        let w: &f32 = outputs.at_3d(0, i, 2)?;
        let h: &f32 = outputs.at_3d(0, i, 3)?;
        let sc: &f32 = outputs.at_3d(0, i, 4)?;

        let mat_size = outputs.mat_size();
        let classes = *mat_size.get(2).unwrap() - 5;
        let mut classes_confidences = vec![];

        for j in 5..5 + classes {
            let confidence: &f32 = outputs.at_3d(0, i, j)?;
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

        let mut x_min = *cx - *w / 2.0;
        let mut y_min = *cy - *h / 2.0;

        x_min /= 640.0;
        y_min /= 640.0;
        let mut width = *w / 640.0;
        let mut height = *h / 640.0;

        //   x_min *= image_width as f32;
        //   y_min *= image_height as f32;
        //   width *= image_width as f32;
        //   height *= image_height as f32;

        // ensure within image bounds
        x_min = x_min.max(0.0).min(1_f32);
        y_min = y_min.max(0.0).min(1_f32);
        width = width.max(0.0).min(1_f32);
        height = height.max(0.0).min(1_f32);

        if sc < min_confidence {
            continue;
        }

        let detection = Detection {
            category: (max_index as u32 + 1).to_string(),
            conf: *sc as f32,
            bbox: [x_min as f32, y_min as f32, width as f32, height as f32],
        };

        detections.push(detection);
    }

    // Non Max Suppression based on category and bounding box area
    let categories: HashSet<String> = HashSet::from_iter(
        detections
            .iter()
            .map(|detection| detection.category.clone()),
    );

    let mut suppressed_detections: Vec<Detection> = vec![];
    for category in categories {
        let mut category_detections: Vec<Detection> = vec![];
        for detection in detections.iter() {
            if detection.category == category {
                category_detections.push(detection.clone());
            }
        }

        let suppressed_detections_for_category = nms(&category_detections, nms_threshold);
        suppressed_detections.extend(suppressed_detections_for_category);
    }

    Ok(suppressed_detections)
}

fn intersection_over_union(bbox1: &[f32; 4], bbox2: &[f32; 4]) -> f32 {
    let x_min1 = bbox1[0];
    let y_min1 = bbox1[1];
    let x_max1 = bbox1[2] + x_min1;
    let y_max1 = bbox1[3] + y_min1;
    let x_min2 = bbox2[0];
    let y_min2 = bbox2[1];
    let x_max2 = bbox2[2] + x_min2;
    let y_max2 = bbox2[3] + y_min2;
    let intersection_x_min = x_min1.max(x_min2);
    let intersection_y_min = y_min1.max(y_min2);
    let intersection_x_max = x_max1.min(x_max2);
    let intersection_y_max = y_max1.min(y_max2);
    let intersection_width = intersection_x_max - intersection_x_min;
    let intersection_height = intersection_y_max - intersection_y_min;
    let intersection_area = intersection_width * intersection_height;
    let bbox1_area = (x_max1 - x_min1) * (y_max1 - y_min1);
    let bbox2_area = (x_max2 - x_min2) * (y_max2 - y_min2);
    let union_area = bbox1_area + bbox2_area - intersection_area;
    intersection_area as f32 / union_area as f32
}

fn nms(detections: &[Detection], nms_threshold: f32) -> Vec<Detection> {
    let mut suppressed_detections: Vec<Detection> = vec![];
    let mut sorted_detections: Vec<Detection> = detections.to_vec();

    sorted_detections.sort_by(|a, b| a.conf.partial_cmp(&b.conf).unwrap());

    for i in 0..sorted_detections.len() {
        let mut keep = true;
        for j in 0..i {
            let bbox1 = &sorted_detections[i].bbox;
            let bbox2 = &sorted_detections[j].bbox;
            let iou = intersection_over_union(bbox1, bbox2);
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
