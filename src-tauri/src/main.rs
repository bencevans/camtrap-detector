use std::{path::Path, sync::Mutex};

use app::{
    structures::{MegaDetectorBatchOutput, MegaDetectorFile},
    utils::find_files,
    yolo,
};

use pathdiff::diff_paths;
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

fn load_model() -> opencv::dnn::Net {
    #[cfg(feature = "builtin")]
    {
        let model_vector: opencv::core::Vector<u8> = include_bytes!("../../md_v5a.0.0.onnx")
            .iter()
            .cloned()
            .collect();

        yolo::load_model_from_bytes(&model_vector).unwrap()
    }

    #[cfg(not(feature = "builtin"))]
    {
        yolo::load_model("/Users/ben/Projects/yolov5-rs/md_v5a.0.0.onnx").unwrap()
    }
}

#[tauri::command]
async fn run_detection(
    base_dir: String,
    relative_paths: bool,
    output_json: Option<String>,
    output_csv: Option<String>,
    output_animals_folder: Option<String>,
    window: Window,
) -> Result<(), String> {
    if output_json.is_none() && output_csv.is_none() && output_animals_folder.is_none() {
        return Err("Expected at least one output format".to_string());
    }

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
        let result = yolo::infer(&mut model, file.as_str(), &0.1, 0.45);

        let file_path = if relative_paths {
            file.as_str().replace(&base_dir, "")
        } else {
            file.as_str().to_string()
        };

        let detections = if let Ok(i) = result {
            i
        } else {
            file_detections.push(MegaDetectorFile {
                file: file_path,
                detections: None,
                error: Some("Unable to read as image".to_string()),
            });

            continue;
        };

        file_detections.push(MegaDetectorFile {
            file: file_path,
            detections: Some(detections),
            error: None,
        });

        let percent = ((index + 1) as f32 / files.len() as f32) * 100.0;

        window.emit("progress", format!("{}%", percent)).unwrap();
    }

    // JSON Output
    if output_json.is_some() && !output_json.as_ref().unwrap().is_empty() {
        window.emit("progress", "Saving JSON").unwrap();
        let output = MegaDetectorBatchOutput {
            images: file_detections.clone(),
            detection_categories: None,
            info: None,
        };
        output.save_json_relative(&base_dir, Path::new(&output_json.unwrap()));
        window.emit("progress", "JSON Saved").unwrap();
    }

    // CSV Output
    if output_csv.is_some() && !output_csv.as_ref().unwrap().is_empty() {
        window.emit("progress", "Saving CSV").unwrap();
        let output = MegaDetectorBatchOutput {
            images: file_detections.clone(),
            detection_categories: None,
            info: None,
        };
        output.save_csv_relative(&base_dir, Path::new(&output_csv.unwrap()));
        window.emit("progress", "CSV Saved").unwrap();
    }

    // Filterd Images Output

    if output_animals_folder.is_some() && !output_animals_folder.as_ref().unwrap().is_empty() {
        window.emit("progress", "Saving Animals").unwrap();
        // create new dir with images containing detections

        for file in file_detections {

            if file.detections.is_none() {
                continue;
            }

            let mut has_animal = false;
            for detection in file.detections.as_ref().unwrap() {
                if detection.category == "1" {
                    has_animal = true;
                    break;
                }
            }

            if !has_animal {
                continue;
            }

            let output_file = Path::new(&output_animals_folder.as_ref().unwrap()).join(
                diff_paths(Path::new(file.file.as_str()), Path::new(&base_dir))
                    .unwrap()
                    .to_str()
                    .unwrap(),
            );

            let output_dir = output_file.parent().unwrap();

            std::fs::create_dir_all(output_dir).unwrap();
            std::fs::copy(file.file.as_str(), output_file).unwrap();
        }
        window.emit("progress", "Animals Saved").unwrap();
    }

    window.emit("progress", "Done").unwrap();

    Ok(())
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
