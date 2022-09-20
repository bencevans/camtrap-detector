use std::path::PathBuf;

use app::{megadetector::load_model, structures::CamTrapImageDetections};

#[tauri::command]
fn is_dir(path: String) -> bool {
    let path = std::path::Path::new(&path);
    path.is_dir()
}

#[tauri::command]
async fn process(path: String, recursive: bool) {
    let files = yolov5cv::helpers::enumerate_images(PathBuf::from(path), recursive);
    let mut model = load_model();

    for file in files {
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

        println!("{:?}", result_handled);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![is_dir, process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
