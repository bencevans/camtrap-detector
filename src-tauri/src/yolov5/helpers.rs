use super::detections::YoloImageDetections;
use image::{ImageError, Rgb};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use walkdir::WalkDir;

/// A set of known image file extensions
static IMAGE_EXTENSIONS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    ["jpg", "jpeg", "png"].iter().cloned().collect()
});

/// Check if path is a known image extension
pub fn is_image_path(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Find all images beleived to be an image.
pub fn enumerate_images(root_dir: PathBuf, recursive: bool) -> Vec<PathBuf> {
    let mut images: Vec<PathBuf> = vec![];

    for entry in WalkDir::new(root_dir)
        .max_depth(if recursive { usize::MAX } else { 1 })
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_image_path(e.path()))
    {
        images.push(entry.into_path());
    }
    images
}

/// Render Bounding Boxes onto an Image and Save
pub fn render_detections(
    image_path: &str,
    detections: &YoloImageDetections,
    output_path: &str,
) -> Result<(), ImageError> {
    let image = image::open(image_path)?;
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

    image.save(output_path)?;

    Ok(())
}
