use image;
use std::{fmt::Error, path::PathBuf};

use crate::structures::{CamTrapDetection, CamTrapImageDetections};

pub enum IncludeCriteria {
    Include,
    Union,
    Exclude,
}

pub struct FilterCriteria {
    animals: IncludeCriteria,
    humans: IncludeCriteria,
    vehicles: IncludeCriteria,
    empty: IncludeCriteria,
}

fn match_criteria(image: &CamTrapImageDetections, criteria: &FilterCriteria) -> bool {
    let mut should_include = false;

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

    match criteria.animals {
        IncludeCriteria::Include => should_include = should_include || has_animals,
        IncludeCriteria::Union => {},
        IncludeCriteria::Exclude => should_include = should_include && !has_animals,
    };

    match criteria.humans {
        IncludeCriteria::Include => should_include = should_include || has_humans,
        IncludeCriteria::Union => {},
        IncludeCriteria::Exclude => should_include = should_include && !has_humans,
    };

    match criteria.vehicles {
        IncludeCriteria::Include => should_include = should_include || has_vehicles,
        IncludeCriteria::Union => {},
        IncludeCriteria::Exclude => should_include = should_include && !has_vehicles,
    };

    match criteria.empty {
        IncludeCriteria::Include => should_include = should_include || has_empty,
        IncludeCriteria::Union => {},
        IncludeCriteria::Exclude => should_include = should_include && !has_empty,
    };

    should_include

    // all but empty: has(animal) || has(human) || has(vehicle)
    // animals but none with humans: has(animal) && no(humans)
}

pub struct DrawCriteria {
    animals: bool,
    humans: bool,
    vehicles: bool,
}

// fn should_draw(image: &mut CamTrapDetection, criteria: &FilterCriteria) -> bool {

// }

pub fn export_image(
    results: Vec<CamTrapImageDetections>,
    output_dir: PathBuf,
    filter_criteria: FilterCriteria,
) -> Result<(), Error> {
    for image in results {
        if !match_criteria(&image, &filter_criteria) {
            continue;
        }

        // let in_image_path = image.file;

        // let out_image_path = output_dir.join(&image.file);
        // let out_image_dir = out_image_path.parent().unwrap();

        // // Create directory / parents if they don't exist
        // std::fs::create_dir_all(&out_image_dir).unwrap();

        // image::open(&in_image_path)?
        //     .save(&out_image_path)?;

        // Read image
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_criteria() {
        let human_only_image = CamTrapImageDetections {
            file: String::from("test.jpg"),
            detections: vec![CamTrapDetection {
                class_index: 1,
                confidence: 0.9,
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            }],
            error: None,
            image_width: None,
            image_height: None,
        };

        let animal_only_image = CamTrapImageDetections {
            file: String::from("test.jpg"),
            detections: vec![CamTrapDetection {
                class_index: 0,
                confidence: 0.9,
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            }],
            error: None,
            image_width: None,
            image_height: None,
        };

        let vehicle_only_image = CamTrapImageDetections {
            file: String::from("test.jpg"),
            detections: vec![CamTrapDetection {
                class_index: 2,
                confidence: 0.9,
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            }],
            error: None,
            image_width: None,
            image_height: None,
        };

        let empty_image = CamTrapImageDetections {
            file: String::from("test.jpg"),
            detections: vec![],
            error: None,
            image_width: None,
            image_height: None,
        };

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

        // All Animals Only
        let criteria = FilterCriteria {
            animals: IncludeCriteria::Include,
            humans: IncludeCriteria::Union,
            vehicles: IncludeCriteria::Union,
            empty: IncludeCriteria::Union,
        };

        assert!(!match_criteria(&human_only_image, &criteria));
        assert!(match_criteria(&animal_only_image, &criteria));
        assert!(!match_criteria(&vehicle_only_image, &criteria));
        assert!(!match_criteria(&empty_image, &criteria));

        // Animals and Empty
        let criteria = FilterCriteria {
            animals: IncludeCriteria::Include,
            humans: IncludeCriteria::Union,
            vehicles: IncludeCriteria::Union,
            empty: IncludeCriteria::Include,
        };

        assert!(!match_criteria(&human_only_image, &criteria));
        assert!(match_criteria(&animal_only_image, &criteria));
        assert!(!match_criteria(&vehicle_only_image, &criteria));
        assert!(match_criteria(&empty_image, &criteria));





    }
}
