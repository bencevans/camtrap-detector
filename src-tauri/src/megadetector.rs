use opencv_yolov5::YoloModel;

pub fn load_model(path: &str) -> YoloModel {
    YoloModel::new_from_file(path, (1280, 1280)).unwrap()
}

pub const CATEGORIES: [&str; 4] = ["Empty", "Animal", "Human", "Vehicle"];