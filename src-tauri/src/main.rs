use std::{sync::Mutex, path::{Path}};

use app::{yolo, utils::find_files, structures::{MegaDetectorFile, MegaDetectorBatchOutput}};

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
        let model_vector: opencv::core::Vector<u8> =
            include_bytes!("/Users/ben/Projects/yolov5-rs/md_v5a.0.0.onnx")
                .iter()
                .cloned()
                .collect();

        yolo::load_model_from_bytes(&model_vector).unwrap()
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
        let detections = yolo::infer(&mut model, file.as_str(), &0.1, 0.45).unwrap();


        let file_path = if relative_paths {
            file.as_str().replace(&base_dir, "")
        } else {
            file.as_str().to_string()
        };

        file_detections.push(MegaDetectorFile {
            file: file_path,
            detections: Some(detections),
            error: None,
        });

        let percent = ((index + 1) as f32 / files.len() as f32) * 100.0;

        window.emit("progress", format!("{}%", percent)).unwrap();
    }

    window.emit("progress", "Saving JSON").unwrap();



    if !output_json.is_empty() {
        let output = MegaDetectorBatchOutput {
            images: file_detections,
            detection_categories: None,
            info: None,
        };
        output.save_json_relative(&base_dir, Path::new(&output_json));
    }

    window.emit("progress", "Done").unwrap();


    // create new dir with images containing detections
    let animal_dir = format!("{}/animal", base_dir);
    std::fs::create_dir_all(animal_dir.as_str()).unwrap();

    for image_file in files {
        let output_file = format!("{}/animal/{}", base_dir, image_file);
        let output_dir = Path::new(&output_file);
        let output_dir = output_dir.parent().unwrap();
        std::fs::create_dir_all(output_dir).unwrap();
        std::fs::copy(image_file.as_str(), output_file.as_str()).unwrap();

    }

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