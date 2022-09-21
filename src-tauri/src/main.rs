use app::{
    exports::{self, csv::CamTrapCSVDetection},
    megadetector::load_model,
    structures::{self, CamTrapImageDetections},
};
use eta::{Eta, TimeAcc};

use std::{path::PathBuf, sync::Mutex};
use tauri::Window;

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
    eta: usize,
}

pub struct AppState(Mutex<App>);

#[derive(Default)]
struct App {
    results: Vec<structures::CamTrapImageDetections>,
}

fn export_csv(results: Vec<structures::CamTrapImageDetections>) {
    tauri::api::dialog::FileDialogBuilder::new()
        .set_file_name("ct.0.1.0.csv")
        .save_file(|file_path| {
            if let Some(file_path) = file_path {
                let mut writer = csv::Writer::from_path(file_path).unwrap();
                for result in results {
                    if let Some(error) = result.error {
                        writer
                            .serialize(CamTrapCSVDetection::new_error(result.file, error))
                            .unwrap();
                    } else if result.detections.is_empty() {
                        writer
                            .serialize(CamTrapCSVDetection::new_empty(result.file))
                            .unwrap();
                    } else {
                        for detection in result.detections {
                            writer
                                .serialize(CamTrapCSVDetection::new_detection(
                                    result.file.clone(),
                                    result.image_width.unwrap(),
                                    result.image_height.unwrap(),
                                    &detection,
                                ))
                                .unwrap();
                        }
                    }
                }

                writer.flush().unwrap();
            } else {
                // user canceled
            }
        })
}

fn export_json(results: Vec<structures::CamTrapImageDetections>) {
    tauri::api::dialog::FileDialogBuilder::new()
        .set_file_name("ct.0.1.0.json")
        .save_file(move |file_path| {
            if let Some(file_path) = file_path {
                let mut writer = std::fs::File::create(file_path).unwrap();
                let json_images: Vec<exports::json::CamTrapJSONImageDetections> =
                    results.into_iter().map(|d| d.into()).collect();
                let json_container = exports::json::CamTrapJSONContainer::new(json_images);

                serde_json::to_writer_pretty(&mut writer, &json_container).unwrap();
            } else {
                // user canceled
            }
        })
}

#[tauri::command]
fn export(format: String, state: tauri::State<'_, AppState>) {
    match format.as_str() {
        "csv" => export_csv(state.0.lock().unwrap().results.clone()),
        "json" => export_json(state.0.lock().unwrap().results.clone()),
        _ => panic!("Unknown export format"),
    }
}

#[tauri::command]
async fn process(
    path: String,
    recursive: bool,
    window: Window,
    state: tauri::State<'_, AppState>,
) -> Result<(), ()> {
    let files = yolov5cv::helpers::enumerate_images(PathBuf::from(path), recursive);
    let files_n = files.len();

    let mut eta = Eta::new(files_n, TimeAcc::SEC);

    window
        .emit(
            "progress",
            Progress {
                current: 0,
                total: files_n,
                percent: 0.0,
                message: String::from("Loading MegaDetector model..."),
                eta: eta.time_remaining(),
            },
        )
        .unwrap();

    let mut model = load_model();
    let mut results: Vec<CamTrapImageDetections> = vec![];

    for (i, file) in files.iter().enumerate() {
        window
            .emit(
                "progress",
                Progress {
                    current: i,
                    total: files_n,
                    percent: eta.progress() * 100.0,
                    eta: eta.time_remaining(),
                    message: String::from("Processing {:?}...")
                        .replace("{:?}", file.to_str().unwrap()),
                },
            )
            .unwrap();
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

    state.0.lock().unwrap().results = results;

    window
        .emit(
            "progress",
            Progress {
                current: files_n,
                total: files_n,
                percent: 100.0,
                message: String::from("Processing Complete"),
                eta: 0,
            },
        )
        .unwrap();

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Default::default()))
        .invoke_handler(tauri::generate_handler![is_dir, process, export])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
