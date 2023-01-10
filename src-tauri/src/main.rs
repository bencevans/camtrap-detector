#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::{
    exports::{
        self,
        csv::CamTrapCSVDetection,
        image::{export_image, DrawCriteria, FilterCriteria},
    },
    megadetector::load_model,
    structures::{self, CamTrapImageDetections},
};
use chug::Chug;
use std::{path::PathBuf, sync::Mutex};
use tauri::{api::dialog, Manager, Window};

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
    path: String,
    message: String,
    eta: Option<usize>,
}

pub struct AppState(Mutex<App>);

#[derive(Default)]
struct App {
    base_dir: PathBuf,
    results: Vec<structures::CamTrapImageDetections>,
}

fn export_csv(
    results: Vec<structures::CamTrapImageDetections>,
    output_path: PathBuf,
) -> Result<(), String> {
    let mut writer = csv::Writer::from_path(output_path).unwrap();

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

    Ok(())
}

fn export_json(
    results: Vec<structures::CamTrapImageDetections>,
    output_path: PathBuf,
) -> Result<(), String> {
    let mut writer = std::fs::File::create(output_path).unwrap();
    let json_images: Vec<exports::json::CamTrapJSONImageDetections> =
        results.into_iter().map(|d| d.into()).collect();
    let json_container = exports::json::CamTrapJSONContainer::new(json_images);

    serde_json::to_writer_pretty(&mut writer, &json_container).unwrap();

    Ok(())
}

#[tauri::command]
async fn export_image_set(
    state: tauri::State<'_, AppState>,
    output_path: PathBuf,
    filter_criteria: FilterCriteria,
    draw_criteria: DrawCriteria,
    window: Window,
) -> Result<(), ()> {
    let results = state.0.lock().unwrap().results.clone();
    let base_dir = state.0.lock().unwrap().base_dir.clone();

    // Ensure it's not the same folder as the raw images
    if output_path == base_dir {
        dialog::message(
            Some(&window),
            "Export Error",
            &"The export folder cannot be the same as the raw images folder.",
        );
        return Err(());
    }

    export_image(
        results,
        base_dir,
        output_path,
        filter_criteria,
        draw_criteria,
    )
    .unwrap();

    dialog::message(
        Some(&window),
        "Image Export Complete",
        &"The image export has completed.",
    );

    Ok(())
}

#[tauri::command]
async fn export(
    format: String,
    output_path: PathBuf,
    state: tauri::State<'_, AppState>,
    window: Window,
) -> Result<(), String> {
    let base_dir = state.0.lock().unwrap().base_dir.clone();

    // Gather the results and convert the paths to relative paths
    let results = state
        .0
        .lock()
        .unwrap()
        .results
        .iter()
        .map(|r| {
            let mut copied = r.clone();
            copied.file = pathdiff::diff_paths(&r.file, &base_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            copied
        })
        .collect();

    let r = match format.as_str() {
        "csv" => export_csv(results, output_path),
        "json" => export_json(results, output_path),
        _ => Err("Unknown export format".to_string()),
    };

    let format_name = match format.as_str() {
        "csv" => "CSV",
        "json" => "JSON",
        _ => "Unknown",
    };

    dialog::message(
        Some(&window),
        "Export Complete",
        &format!("The {} export has completed.", format_name),
    );

    r
}

#[tauri::command]
async fn process(
    path: String,
    confidence_threshold: f32,
    recursive: bool,
    window: Window,
    state: tauri::State<'_, AppState>,
    handle: tauri::AppHandle,
) -> Result<(), ()> {
    let files = opencv_yolov5::helpers::enumerate_images(PathBuf::from(&path), recursive);
    let files_n = files.len();

    println!("Running with Confidence Threshold {}", confidence_threshold);

    window
        .emit(
            "progress",
            Progress {
                current: 0,
                total: files_n,
                percent: 0.0,
                message: String::from("Loading MegaDetector model..."),
                path: String::from(""),
                eta: None,
            },
        )
        .unwrap();

    let mut model = load_model(
        handle
            .path_resolver()
            .resolve_resource("../md_v5a.0.0.onnx")
            .unwrap()
            .to_str()
            .unwrap(),
    );

    let mut eta = Chug::new(100, files_n);
    let mut results: Vec<CamTrapImageDetections> = vec![];

    for (i, file) in files.iter().enumerate() {
        window
            .emit(
                "progress",
                Progress {
                    current: i,
                    total: files_n,
                    percent: (i as f64 / files_n as f64) * 100.0,
                    eta: eta.eta().map(|eta| eta.as_secs() as usize),
                    path: file.to_str().unwrap().to_string(),
                    message: String::from("Processing "),
                },
            )
            .unwrap();
        eta.tick();

        let result = model.detect(file.to_str().unwrap(), confidence_threshold, 0.45);

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

    state.0.lock().unwrap().base_dir = PathBuf::from(&path);
    state.0.lock().unwrap().results = results;

    window
        .emit(
            "progress",
            Progress {
                current: files_n,
                total: files_n,
                percent: 100.0,
                message: String::from("Processing Complete"),
                path: String::from(""),
                eta: None,
            },
        )
        .unwrap();

    Ok(())
}

#[tauri::command]
async fn showup(window: Window) {
    window.get_window("main").unwrap().show().unwrap(); // replace "main" by the name of your window
}

fn main() {
    let mut context = tauri::generate_context!();

    let update_url = if cfg!(feature = "cuda") {
        "https://releases.camtrap.net/detector/{{target}}/{{current_version}}"
    } else {
        "https://releases.camtrap.net/detector-cuda/{{target}}/{{current_version}}"
    };

    let updater = &mut context.config_mut().tauri.updater;
    let urls = vec![tauri::utils::config::UpdaterEndpoint(
        update_url.parse().unwrap(),
    )];
    updater.endpoints.replace(urls);

    tauri::Builder::default()
        .manage(AppState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            is_dir,
            process,
            export,
            export_image_set,
            showup
        ])
        .run(context)
        .expect("error while running tauri application");
}
