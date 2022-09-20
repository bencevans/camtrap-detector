use yolov5cv::YoloModel;

pub fn load_model() -> YoloModel {
    let model_vector: opencv::core::Vector<u8> = include_bytes!("../../md_v5a.0.0.onnx")
        .iter()
        .cloned()
        .collect();

    YoloModel::new_from_buffer(&model_vector, (640, 640)).unwrap()
}
