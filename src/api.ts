import { invoke } from "@tauri-apps/api/tauri";

export async function process(
  path: string,
  confidenceThreshold: number,
  recursive: boolean,
) {
  return await invoke("process", {
    path,
    confidenceThreshold,
    recursive,
  });
}

export async function createExport(format: string, outputPath: string) {
  return await invoke("export", { format, outputPath });
}

type FilterCriteriaOption = "Include" | "Intersect" | "Exclude";

interface FilterCriteria {
  animals: FilterCriteriaOption;
  humans: FilterCriteriaOption;
  vehicles: FilterCriteriaOption;
  empty: FilterCriteriaOption;
}

export function createFilterCriteria(
  animals: FilterCriteriaOption,
  humans: FilterCriteriaOption,
  vehicles: FilterCriteriaOption,
  empty: FilterCriteriaOption,
): FilterCriteria {
  return {
    animals: animals,
    humans: humans,
    vehicles: vehicles,
    empty: empty,
  };
}

interface DrawCriteria {
  animals: boolean;
  humans: boolean;
  vehicles: boolean;
}

export function createDrawCriteria(
  animals: boolean,
  humans: boolean,
  vehicles: boolean,
) {
  return {
    animals: animals,
    humans: humans,
    vehicles: vehicles,
  };
}

export function exportImageSet(
  outputPath: string,
  filterCriteria: FilterCriteria,
  drawCriteria: DrawCriteria,
) {
  return invoke("export_image_set", {
    outputPath,
    filterCriteria: filterCriteria,
    drawCriteria: drawCriteria,
  });
}
