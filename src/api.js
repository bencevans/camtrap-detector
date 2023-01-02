import { invoke } from "@tauri-apps/api/tauri";

export function process(path, confidenceThreshold, recursive) {
  return invoke("process", {
    path,
    confidenceThreshold,
    recursive,
  });
}

export function createExport(format, outputPath) {
  return invoke("export", { format, outputPath });
}

export const IncludeCriteria = {
  Include: "Include",
  Union: "Union",
  Exclude: "Exclude",
};

export function createFilterCriteria(animals, humans, vehicles, empty) {
  return {
    animals: animals,
    humans: humans,
    vehicles: vehicles,
    empty: empty,
  };
}

export function createDrawCriteria(animals, humans, vehicles) {
  return {
    animals: animals,
    humans: humans,
    vehicles: vehicles,
  };
}

export function exportImageSet(outputPath, filterCriteria, drawCriteria) {
  return invoke("export_image_set", {
    outputPath,
    filterCriteria: filterCriteria,
    drawCriteria: drawCriteria,
  });
}
