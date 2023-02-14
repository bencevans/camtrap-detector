//! Magic Image module
//!
//! This module contains the logic to load, draw bounding boxes and save images while persisting the original EXIF data.
//!
//! The module is based on the [image](https://crates.io/crates/image) and [img_parts](https://crates.io/crates/img_parts) crates.
//!
//! # Example
//!
//! ```
//! use magic_image::MagicImage;
//!
//! let mut image = MagicImage::open("path/to/image.jpg").unwrap();
//!
//! image.draw_bounding_box(0.1, 0.1, 0.2, 0.2, [255, 0, 0, 255]);
//! image.draw_bounding_box(0.3, 0.3, 0.2, 0.2, [0, 255, 0, 255]);
//!
//! image.save("path/to/output.jpg").unwrap();
//! ```

use img_parts::ImageEXIF;
use std::fs;
use std::io::Cursor;
use std::path::Path;

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

    /// Image width
    pub fn width(&self) -> u32 {
        self.image.width()
    }

    /// Image height
    pub fn height(&self) -> u32 {
        self.image.height()
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
