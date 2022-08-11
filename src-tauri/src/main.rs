use std::sync::Mutex;

use app::yolo;

pub struct AppState {
    pub root_path: String,
}

pub struct AppStateMutex(Mutex<AppState>);

impl AppState {
    pub fn new(root_path: String) -> Self {
        AppState { root_path }
    }

    pub fn get_root_path(&self) -> &str {
        &self.root_path
    }

    pub fn get_root_path_mut(&mut self) -> &mut String {
        &mut self.root_path
    }

    pub fn set_root_path(&mut self, root_path: String) {
        self.root_path = root_path;
    }
}

#[tauri::command]
async fn run_detection(base_dir: String) {
    let extentions = vec!["jpg", "png", "JPG", "PNG", "jpeg", "JPEG"];
    let mut files = Vec::new();

    println!("Enumerating files in {}", base_dir);

    for entry in std::fs::read_dir(base_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let ext = path.to_str().unwrap().split('.').last().unwrap();

        if extentions.contains(&ext) {
            files.push(path.clone());
        }
    }

    println!("Loading Model");

    #[cfg(feature = "builtin")]
    let mut model = {
        let model_bytes = include_bytes!("/Users/ben/Projects/yolov5-rs/md_v5a.0.0.onnx");
        let MODEL_VECTOR: opencv::core::Vector<u8> = model_bytes.iter().cloned().collect();

        yolo::load_model_from_bytes(&MODEL_VECTOR).unwrap()
    };

    #[cfg(not(feature = "builtin"))]
    let mut model = {
        yolo::load_model("/Users/ben/Projects/yolov5-rs/md_v5a.0.0.onnx").unwrap()
    };

    println!("Running Detection");

    // let mut results = Vec::new();
    // let mut count = 0;

    for file in files {
        let detections = yolo::infer(&mut model, file.to_str().unwrap(), &0.5, 0.4).unwrap();
        println!("{:?}", detections);
        println!("{}", file.to_str().unwrap());
    }
}

// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

fn main() {
    tauri::Builder::default()
        .manage(AppStateMutex(Mutex::new(AppState::new(String::from(".")))))
        .invoke_handler(tauri::generate_handler!(run_detection))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
