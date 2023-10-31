use super::detections::YoloImageDetections;
use image::Rgb;
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

const IMAGE_EXTENTIONS: [&str; 3] = ["jpg", "jpeg", "png"];

/// Check if path is a known image extention
pub fn is_image_path(path: &Path) -> bool {
    match path.extension() {
        None => false,
        Some(a) => IMAGE_EXTENTIONS.contains(&a.to_str().unwrap().to_lowercase().as_str()),
    }
}

/// Find all images beleived to be an image.
pub fn enumerate_images(root_dir: PathBuf, recursive: bool) -> Vec<PathBuf> {
    let mut images: Vec<PathBuf> = vec![];

    for entry in WalkDir::new(root_dir)
        .max_depth(if recursive {::std::usize::MAX} else {1})
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_image_path(e.path())) {
            images.push(entry.into_path());
        }
    images
}

/// Render Bounding Boxes onto an Image and Save
pub fn render_detections(
    image_path: &str,
    detections: &YoloImageDetections,
    output_path: &str,
) -> Result<(), ()> {
    let image = image::open(image_path).unwrap();
    let mut image = image.to_rgb8();

    for detection in &detections.detections {
        let x = (detection.x) * image.width() as f32;
        let y = (detection.y) * image.height() as f32;
        let width = (detection.width) * image.width() as f32;
        let height = (detection.height) * image.height() as f32;

        draw_hollow_rect_mut(
            &mut image,
            Rect::at(x as i32, y as i32).of_size(width as u32, height as u32),
            Rgb([255u8, 0u8, 0u8]),
        );
    }

    image.save(output_path).unwrap();

    Ok(())
}

/// Checks if CUDA and a Supported CUDA device can be found.
pub fn is_cuda_available() -> bool {
    println!("Warning: is_cuda_available is not implemented");
    false
}
