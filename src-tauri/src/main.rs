use std::{sync::Mutex, path::Path};

use app::yolo;
use serde::{Serialize, Deserialize};
use tauri::Window;

pub struct AppState {
    pub root_path: String,
}

pub struct AppStateMutex(Mutex<AppState>);

impl AppState {
    pub fn new(root_path: String) -> Self {
        AppState { root_path }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MDBatchFile {
    pub path: String,
    pub detections: Vec<yolo::BoxDetection>,
}

fn find_files(root_path: &Path, extentions: &[&str], recursive_search: bool) -> Vec<String> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(root_path).unwrap() {

        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let ext = path.extension();
            if let Some(ext) = ext {
                if extentions.contains(&ext.to_str().unwrap()) {
                    files.push(path.to_str().unwrap().to_string());
                }
            }
        } else if path.is_dir() && recursive_search {
            files.append(&mut find_files(&path, extentions, recursive_search));
        }
    }

    files
}

fn load_model() -> opencv::dnn::Net {
    #[cfg(feature = "builtin")]
    {
        let MODEL_VECTOR: opencv::core::Vector<u8> =
            include_bytes!("/Users/ben/Projects/yolov5-rs/md_v5a.0.0.onnx")
                .iter()
                .cloned()
                .collect();

        yolo::load_model_from_bytes(&MODEL_VECTOR).unwrap()
    }

    #[cfg(not(feature = "builtin"))]
    { yolo::load_model("/Users/ben/Projects/yolov5-rs/md_v5a.0.0.onnx").unwrap() }
}

#[tauri::command]
async fn run_detection(base_dir: String, relative_paths: bool, output_json: String,  window: Window) {
    let extentions = vec!["jpg", "png", "JPG", "PNG", "jpeg", "JPEG"];

    // Search for all images with known extentions
    window.emit("progress", "Enumerating Images...").unwrap();
    let files = find_files(Path::new(&base_dir), &extentions, true);

    // Load the model
    window.emit("progress", "Loading Model").unwrap();
    let mut model = load_model();
    window.emit("progress", "Model Loaded").unwrap();

    // Run the detection on all images
    let mut file_detections = vec![];

    for (index, file) in files.iter().enumerate() {
        let detections = yolo::infer(&mut model, file.as_str(), &0.5, 0.4).unwrap();
        println!("{:?}", detections);
        println!("{}", file.as_str());

        file_detections.push(MDBatchFile {
            path: file.as_str().to_string(),
            detections,
        });

        let percent = ((index + 1) as f32 / files.len() as f32) * 100.0;

        window.emit("progress", format!("{}%", percent)).unwrap();
    }

    window.emit("progress", "Saving JSON").unwrap();

    println!("{:?}", file_detections);

    let output_path = format!("{}/output.json", &base_dir);
    let output_file = std::fs::File::create(output_path).unwrap();
    let mut output_writer = std::io::BufWriter::new(output_file);
    serde_json::to_writer_pretty(&mut output_writer, &file_detections).unwrap();

    window.emit("progress", "Done").unwrap();

}


fn main() {
    tauri::Builder::default()
        .manage(AppStateMutex(Mutex::new(AppState::new(String::from(".")))))
        .invoke_handler(tauri::generate_handler!(run_detection))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn finding_files() {
        let root_path = Path::new("./vendor/opencv/modules/highgui/src/files_Qt/Milky");
        let extentions = vec!["jpg", "png", "JPG", "PNG", "jpeg", "JPEG"];

        let files = super::find_files(root_path, &extentions, false);
        assert_eq!(files.len(), 0);

        let files = super::find_files(root_path, &extentions, true);
        assert_eq!(files.len(), 262);

    }
}