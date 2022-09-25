import { invoke } from "@tauri-apps/api/tauri";

export function process(path, recursive) {
  return invoke("process", {
    path,
    recursive,
  });
}

export function createExport(format) {
  return invoke("export", { format });
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

export function exportImageSet(filterCriteria, drawCriteria) {
  return invoke("export_image_set", {
    filterCriteria: filterCriteria,
    drawCriteria: drawCriteria,
  });
}
