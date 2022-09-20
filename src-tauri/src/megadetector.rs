use yolov5cv::YoloModel;

const MODEL_PATH: &str = "../../md_v5a.0.0.onnx";
const MODEL_INPUT_SIZE: (i32, i32) = (240, 240);

pub fn load_model() -> YoloModel {
  #[cfg(feature = "builtin")]
  {
      let model_vector: opencv::core::Vector<u8> = include_bytes!(MODEL_PATH)
          .iter()
          .cloned()
          .collect();

      YoloModel::new_from_buffer(&model_vector).unwrap()
  }

  #[cfg(not(feature = "builtin"))]
  {
      YoloModel::new_from_file(
          MODEL_PATH,
          MODEL_INPUT_SIZE,
      )
      .unwrap()
  }
}
