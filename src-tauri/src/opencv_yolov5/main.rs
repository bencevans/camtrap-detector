#![cfg(feature = "cli")]
use std::path::PathBuf;

use opencv_yolov5::{helpers::enumerate_images, YoloImageDetections, YoloModel};

#[derive(clap::Parser)]
struct Cli {
    model_path: PathBuf,

    // #[clap(parse(from_os_str))]
    root_path: PathBuf,

    #[clap(long, default_value = "false")]
    recursive: bool,

    #[clap(long, default_value = "640")]
    input_width: i32,

    #[clap(long, default_value = "640")]
    input_height: i32,
}

fn main() {
    let mut args = <Cli as clap::Parser>::parse();

    // Handle ~ in paths
    args.model_path = args.model_path.canonicalize().unwrap();
    args.root_path = args.root_path.canonicalize().unwrap();

    let model_progress = indicatif::ProgressBar::new_spinner();
    let mut model = YoloModel::new_from_file(
        args.model_path.to_str().unwrap(),
        (args.input_width, args.input_height),
    )
    .expect("Unable to load model.");
    model_progress.finish_with_message("Model loaded.");

    let images = enumerate_images(args.root_path, true);

    let image_progress = indicatif::ProgressBar::new(images.len() as u64);
    image_progress.set_style(
        indicatif::ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} {per_sec} ({eta_precise})",
            )
            .unwrap()
            .progress_chars("=> "),
    );

    let mut results: Vec<YoloImageDetections> = vec![];

    for image_path in images {
        image_progress.inc(1);

        let detections = model
            .detect(image_path.to_str().unwrap(), 0.1, 0.45)
            .unwrap();

        results.push(detections);
    }

    image_progress.finish_with_message("Done.");

    std::fs::write(
        "output.json",
        serde_json::to_string_pretty(&results).unwrap(),
    )
    .expect("Failed to write results");
}
