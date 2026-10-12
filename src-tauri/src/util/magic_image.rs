//! Magic Image module
//!
//! This module contains the logic to load, draw bounding boxes and save images while persisting the original EXIF data.
//!
//! The module is based on the [image](https://crates.io/crates/image) and [img_parts](https://crates.io/crates/img_parts) crates.
//!

use img_parts::ImageEXIF;
use std::fs;
use std::io::Cursor;
use std::path::Path;

/// Magic Image
///
/// This struct represents an image that can be drawn on and saved while preserving the original EXIF data.
pub(crate) struct MagicImage {
    image: image::DynamicImage,
    exif: Option<img_parts::Bytes>,
    original_format: image::ImageFormat,
}

impl MagicImage {
    /// Open an image from a path
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let image_bytes = fs::read(path.as_ref())?;

        let original_format = image::guess_format(&image_bytes)?;
        let image = image::load_from_memory_with_format(&image_bytes, original_format)?;

        let exif = match original_format {
            image::ImageFormat::Jpeg => {
                img_parts::jpeg::Jpeg::from_bytes(image_bytes.into())?.exif()
            }
            image::ImageFormat::Png => img_parts::png::Png::from_bytes(image_bytes.into())?.exif(),
            _ => None,
        };

        Ok(Self {
            image,
            exif,
            original_format,
        })
    }

    /// Draw a bounding box on the image
    pub fn draw_bounding_box(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color: image::Rgba<u8>,
    ) {
        let rect = imageproc::rect::Rect::at(x, y).of_size(width, height);
        imageproc::drawing::draw_hollow_rect_mut(&mut self.image, rect, color);
    }

    /// Save the image to a path (preserving EXIF data if it is a JPEG or PNG)
    ///
    /// Preserves the original image format no matter what the path extension is.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        // First, we need to check if the image is a JPEG or PNG. If it is, we need to use img_parts to
        // preserve the EXIF data. Otherwise, we can use the image crate to save the image.
        let mut output_file = fs::File::create(path)?;

        let mut buffer = Vec::new();
        let mut cursored_buffer = Cursor::new(&mut buffer);

        self.image
            .write_to(&mut cursored_buffer, self.original_format)?;

        match self.original_format {
            image::ImageFormat::Jpeg => {
                let mut jpeg = img_parts::jpeg::Jpeg::from_bytes(buffer.into())?;
                jpeg.set_exif(self.exif.clone());
                // img-parts inserts EXIF at a fixed index, which can put it after SOF.
                // ExifPro stops searching at SOF, so place EXIF immediately after SOI.
                let segments = jpeg.segments_mut();
                if let Some(index) = segments.iter().position(|segment| {
                    segment.marker() == img_parts::jpeg::markers::APP1
                        && segment.contents().starts_with(b"Exif\0\0")
                }) {
                    let exif = segments.remove(index);
                    segments.insert(0, exif);
                }
                jpeg.encoder().write_to(&mut output_file)?;
            }
            image::ImageFormat::Png => {
                let mut png = img_parts::png::Png::from_bytes(buffer.into())?;
                png.set_exif(self.exif.clone());
                png.encoder().write_to(&mut output_file)?;
            }
            _ => {
                self.image
                    .write_to(&mut output_file, self.original_format)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_OUTPUT: AtomicUsize = AtomicUsize::new(0);

    fn save_and_read(image: &MagicImage) -> Vec<u8> {
        let path = std::env::temp_dir().join(format!(
            "camtrap-exif-test-{}-{}",
            std::process::id(),
            NEXT_OUTPUT.fetch_add(1, Ordering::Relaxed)
        ));
        image.save(&path).unwrap();
        let bytes = fs::read(&path).unwrap();
        fs::remove_file(path).unwrap();
        bytes
    }

    // Match ExifPro's header scan: it skips APPn, COM and DQT, but stops at SOF.
    fn exifpro_finds_exif(jpeg: &img_parts::jpeg::Jpeg) -> bool {
        for segment in jpeg.segments() {
            let marker = segment.marker();
            if marker == 0xe1 && segment.contents().starts_with(b"Exif\0\0") {
                return true;
            }
            if !(0xe0..=0xef).contains(&marker) && marker != 0xfe && marker != 0xdb {
                return false;
            }
        }
        false
    }

    #[test]
    fn jpeg_export_preserves_exif_where_exifpro_can_find_it() {
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/dataset/IMG_0089_peccary.JPG");
        let original =
            img_parts::jpeg::Jpeg::from_bytes(fs::read(&fixture).unwrap().into()).unwrap();
        let original_exif = original.exif().expect("Fixture must contain camera EXIF");

        for draw_boxes in [false, true] {
            let mut image = MagicImage::open(&fixture).unwrap();
            if draw_boxes {
                image.draw_bounding_box(10, 10, 100, 100, image::Rgba([255, 0, 0, 255]));
            }
            let bytes = save_and_read(&image);
            let decoded = image::load_from_memory(&bytes).unwrap();
            assert_eq!(decoded.width(), image.image.width());
            assert_eq!(decoded.height(), image.image.height());
            let exported = img_parts::jpeg::Jpeg::from_bytes(bytes.into()).unwrap();
            assert_eq!(exported.exif(), Some(original_exif.clone()));
            assert!(exifpro_finds_exif(&exported));
        }
    }

    #[test]
    fn jpeg_without_exif_still_exports_without_exif() {
        let image = MagicImage {
            image: image::DynamicImage::new_rgb8(16, 16),
            exif: None,
            original_format: image::ImageFormat::Jpeg,
        };
        let bytes = save_and_read(&image);
        assert!(image::load_from_memory(&bytes).is_ok());
        let exported = img_parts::jpeg::Jpeg::from_bytes(bytes.into()).unwrap();
        assert_eq!(exported.exif(), None);
    }

    #[test]
    fn png_export_still_preserves_exif() {
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/dataset/IMG_0089_peccary.JPG");
        let original_exif = MagicImage::open(fixture).unwrap().exif.unwrap();
        let image = MagicImage {
            image: image::DynamicImage::new_rgb8(16, 16),
            exif: Some(original_exif.clone()),
            original_format: image::ImageFormat::Png,
        };
        let bytes = save_and_read(&image);
        assert!(image::load_from_memory(&bytes).is_ok());
        let exported = img_parts::png::Png::from_bytes(bytes.into()).unwrap();
        assert_eq!(exported.exif(), Some(original_exif));
    }
}
