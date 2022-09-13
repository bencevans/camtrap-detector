use std::path::{ PathBuf, Path};

use image::Rgb;
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

use crate::detections::YoloImageDetections;


const IMAGE_EXTENTIONS: [&str; 3] = ["jpg", "jpeg", "png"];

pub fn is_image_path(path: &Path) -> bool {
    match path.extension() {
        None => false,
        Some(a) => IMAGE_EXTENTIONS.contains(&a.to_str().unwrap().to_lowercase().as_str()),
    }
}

pub fn enumerate_images(root_dir: PathBuf, recursive: bool) -> Vec<PathBuf> {
    if root_dir.is_file() {
        if is_image_path(&root_dir) {
            vec![root_dir]
        } else {
            vec![]
        }
    } else {
        let mut images: Vec<PathBuf> = vec![];

        for entry in root_dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() && recursive {
                images.extend(enumerate_images(path, recursive));
            } else if path.is_file() && is_image_path(&path) {
                images.push(path);
            }
        }

        images
    }
}

pub fn render_detections(
    image_path: &str,
    detections: &YoloImageDetections,
    output_path: &str,
) -> Result<(), opencv::Error> {
    let image = image::open(image_path).unwrap();
    let mut image = image.to_rgb8();

    for detection in &detections.detections {
        println!("{:?}", detection);

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
