use crate::structures::{CamTrapDetection, CamTrapImageDetections};
use crate::util::magic_image::MagicImage;
use image::Rgba;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub enum IncludeCriteria {
    Include,
    Intersect,
    Exclude,
}

#[derive(Serialize, Deserialize)]
pub struct FilterCriteria {
    animals: IncludeCriteria,
    humans: IncludeCriteria,
    vehicles: IncludeCriteria,
    empty: IncludeCriteria,
}

/// Check if an image matches the filter criteria
fn match_criteria(image: &CamTrapImageDetections, criteria: &FilterCriteria) -> bool {
    let mut has_animals = false;
    let mut has_humans = false;
    let mut has_vehicles = false;
    let has_empty = image.detections.is_empty();

    for detection in &image.detections {
        match detection.class_index {
            0 => has_animals = true,
            1 => has_humans = true,
            2 => has_vehicles = true,
            _ => {}
        }
    }

    let should_include = if let IncludeCriteria::Include = criteria.animals {
        has_animals
    } else {
        false
    } || if let IncludeCriteria::Include = criteria.humans {
        has_humans
    } else {
        false
    } || if let IncludeCriteria::Include = criteria.vehicles {
        has_vehicles
    } else {
        false
    } || if let IncludeCriteria::Include = criteria.empty {
        has_empty
    } else {
        false
    };

    let should_exclude = if let IncludeCriteria::Exclude = criteria.animals {
        has_animals
    } else {
        false
    } || if let IncludeCriteria::Exclude = criteria.humans {
        has_humans
    } else {
        false
    } || if let IncludeCriteria::Exclude = criteria.vehicles {
        has_vehicles
    } else {
        false
    } || if let IncludeCriteria::Exclude = criteria.empty {
        has_empty
    } else {
        false
    };

    should_include && !should_exclude
}

#[derive(Serialize, Deserialize)]
pub struct DrawCriteria {
    animals: bool,
    humans: bool,
    vehicles: bool,
}

/// Check if a detection should be drawn
fn should_draw(detection: &CamTrapDetection, criteria: &DrawCriteria) -> bool {
    match detection.class_index {
        0 => criteria.animals,
        1 => criteria.humans,
        2 => criteria.vehicles,
        _ => false,
    }
}

pub fn export_image(
    results: Vec<CamTrapImageDetections>,
    base_dir: PathBuf,
    output_dir: PathBuf,
    filter_criteria: FilterCriteria,
    draw_criteria: DrawCriteria,
) -> Result<(), Box<dyn std::error::Error>> {
    // Filter out images that don't match the criteria
    let results: Vec<_> = results
        .into_iter()
        .filter(|image| match_criteria(image, &filter_criteria))
        .collect();

    // TODO: Parallelisee this
    for image_meta in results.iter() {
        let mut image = MagicImage::open(&image_meta.file).unwrap();

        for detection in &image_meta.detections {
            if should_draw(detection, &draw_criteria) {
                let color = match detection.class_index {
                    0 => Rgba([255, 255, 255, 255]),
                    1 => Rgba([255, 0, 0, 255]),
                    2 => Rgba([0, 0, 255, 255]),
                    _ => Rgba([0, 0, 0, 255]),
                };

                image.draw_bounding_box(
                    (detection.x * image.width() as f32) as i32,
                    (detection.y * image.height() as f32) as i32,
                    (detection.width * image.width() as f32) as u32,
                    (detection.height * image.height() as f32) as u32,
                    color,
                );
            }
        }

        let image_rel_path = pathdiff::diff_paths(&image_meta.file, &base_dir).unwrap();

        let out_image_path = output_dir.join(image_rel_path);
        let out_image_dir = out_image_path.parent().unwrap();

        // Create directory / parents if they don't exist
        std::fs::create_dir_all(out_image_dir).unwrap();

        image
            .save(out_image_path)
            .unwrap();
    }

   
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_image(class_indexes: Vec<u32>) -> CamTrapImageDetections {
        CamTrapImageDetections {
            file: String::from("test.jpg"),
            detections: class_indexes
                .iter()
                .map(|i| CamTrapDetection {
                    class_index: *i,
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                    confidence: 1.0,
                })
                .collect(),
            error: None,
            image_width: None,
            image_height: None,
        }
    }

    #[test]
    fn test_match_criteria() {
        let animal_only_image = create_image(vec![0]);
        let human_only_image = create_image(vec![1]);
        let vehicle_only_image = create_image(vec![2]);
        let empty_image = create_image(vec![]);

        let animal_and_human_image = create_image(vec![0, 1]);
        let animal_and_vehicle_image = create_image(vec![0, 2]);
        let human_and_vehicle_image = create_image(vec![1, 2]);
        let animal_human_and_vehicle_image = create_image(vec![0, 1, 2]);

        // Include Everything
        let criteria = FilterCriteria {
            animals: IncludeCriteria::Include,
            humans: IncludeCriteria::Include,
            vehicles: IncludeCriteria::Include,
            empty: IncludeCriteria::Include,
        };

        assert!(match_criteria(&human_only_image, &criteria));
        assert!(match_criteria(&animal_only_image, &criteria));
        assert!(match_criteria(&vehicle_only_image, &criteria));
        assert!(match_criteria(&empty_image, &criteria));
        assert!(match_criteria(&animal_and_human_image, &criteria));
        assert!(match_criteria(&animal_and_vehicle_image, &criteria));
        assert!(match_criteria(&human_and_vehicle_image, &criteria));
        assert!(match_criteria(&animal_human_and_vehicle_image, &criteria));

        // Exclude Everything
        let criteria = FilterCriteria {
            animals: IncludeCriteria::Exclude,
            humans: IncludeCriteria::Exclude,
            vehicles: IncludeCriteria::Exclude,
            empty: IncludeCriteria::Exclude,
        };

        assert!(!match_criteria(&human_only_image, &criteria));
        assert!(!match_criteria(&animal_only_image, &criteria));
        assert!(!match_criteria(&vehicle_only_image, &criteria));
        assert!(!match_criteria(&empty_image, &criteria));
        assert!(!match_criteria(&animal_and_human_image, &criteria));
        assert!(!match_criteria(&animal_and_vehicle_image, &criteria));
        assert!(!match_criteria(&human_and_vehicle_image, &criteria));
        assert!(!match_criteria(&animal_human_and_vehicle_image, &criteria));

        // All Animals
        let criteria = FilterCriteria {
            animals: IncludeCriteria::Include,
            humans: IncludeCriteria::Intersect,
            vehicles: IncludeCriteria::Intersect,
            empty: IncludeCriteria::Intersect,
        };

        assert!(!match_criteria(&human_only_image, &criteria));
        assert!(match_criteria(&animal_only_image, &criteria));
        assert!(!match_criteria(&vehicle_only_image, &criteria));
        assert!(!match_criteria(&empty_image, &criteria));
        assert!(match_criteria(&animal_and_human_image, &criteria));
        assert!(match_criteria(&animal_and_vehicle_image, &criteria));
        assert!(!match_criteria(&human_and_vehicle_image, &criteria));
        assert!(match_criteria(&animal_human_and_vehicle_image, &criteria));

        // Animals and Empty
        let criteria = FilterCriteria {
            animals: IncludeCriteria::Include,
            humans: IncludeCriteria::Intersect,
            vehicles: IncludeCriteria::Intersect,
            empty: IncludeCriteria::Include,
        };

        assert!(!match_criteria(&human_only_image, &criteria));
        assert!(match_criteria(&animal_only_image, &criteria));
        assert!(!match_criteria(&vehicle_only_image, &criteria));
        assert!(match_criteria(&empty_image, &criteria));
        assert!(match_criteria(&animal_and_human_image, &criteria));
        assert!(match_criteria(&animal_and_vehicle_image, &criteria));
        assert!(!match_criteria(&human_and_vehicle_image, &criteria));
        assert!(match_criteria(&animal_human_and_vehicle_image, &criteria));

        // Animals but none with humans
        let criteria = FilterCriteria {
            animals: IncludeCriteria::Include,
            humans: IncludeCriteria::Exclude,
            vehicles: IncludeCriteria::Intersect,
            empty: IncludeCriteria::Intersect,
        };

        assert!(!match_criteria(&human_only_image, &criteria));
        assert!(match_criteria(&animal_only_image, &criteria));
        assert!(!match_criteria(&vehicle_only_image, &criteria));
        assert!(!match_criteria(&empty_image, &criteria));
        assert!(!match_criteria(&animal_and_human_image, &criteria));
        assert!(match_criteria(&animal_and_vehicle_image, &criteria));
        assert!(!match_criteria(&human_and_vehicle_image, &criteria));
        assert!(!match_criteria(&animal_human_and_vehicle_image, &criteria));
    }
}
