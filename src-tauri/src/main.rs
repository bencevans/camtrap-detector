use std::path::PathBuf;

use app::{megadetector::load_model, structures::CamTrapImageDetections};
use tauri::{Window};
use eta::{Eta,TimeAcc};

#[tauri::command]
fn is_dir(path: String) -> bool {
    let path = std::path::Path::new(&path);
    path.is_dir()
}

#[derive(serde::Serialize, Clone)]
struct Progress {
    current: usize,
    total: usize,
    percent: f64,
    message: String,
    eta: usize
}

#[tauri::command]
async fn process(path: String, recursive: bool, window: Window) {
    let files = yolov5cv::helpers::enumerate_images(PathBuf::from(path), recursive);
    let files_n = files.len();

    let mut eta = Eta::new(files_n, TimeAcc::SEC);

    window.emit("progress", Progress {
        current: 0,
        total: files_n,
        percent: 0.0,
        message: String::from("Loading MegaDetector model..."),
        eta: eta.time_remaining()
    }).unwrap();

    let mut model = load_model();
    let mut results: Vec<CamTrapImageDetections> = vec![];

    for (i, file) in files.iter().enumerate() {
        window.emit("progress", Progress {
            current: i,
            total: files_n,
            percent: eta.progress() * 100.0,
            eta: eta.time_remaining(),
            message: String::from("Processing {:?}...").replace("{:?}", file.to_str().unwrap())
        }).unwrap();
        eta.step();
        

        let result = model.detect(file.to_str().unwrap(), 0.1, 0.1);
        let result_handled = match result {
            Ok(result) => result.into(),
            Err(err) => CamTrapImageDetections {
                file: file.to_str().unwrap().to_string(),
                error: Some(err.to_string()),
                image_width: None,
                image_height: None,
                detections: vec![],
            },
        };

        results.push(result_handled);
    }

    window.emit("progress", Progress {
        current: files_n,
        total: files_n,
        percent: 100.0,
        message: String::from("Processing Complete"),
        eta: 0
    }).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![is_dir, process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
