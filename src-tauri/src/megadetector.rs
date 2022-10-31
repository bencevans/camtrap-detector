use yolov5cv::YoloModel;

pub fn load_model(path: &str) -> YoloModel {
    YoloModel::new_from_file(path, (640, 640)).unwrap()
}

pub const CATEGORIES: [&str; 4] = ["Empty", "Animal", "Human", "Vehicle"];