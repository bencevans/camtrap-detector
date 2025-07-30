import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export async function showWindow() {
  return await invoke("showup");
}

/**
 * Check if a path is a directory
 */
export async function isDir(path: string): Promise<boolean> {
  return await invoke("is_dir", { path });
}

export interface ProgressReport {
  current: number;
  total: number;
  percent: number;
  message: string;
  path: string;
  eta: number | null;
}

export async function listenProgress(
  onProgress: (report: ProgressReport) => void
) {
  return await listen("progress", (event) => {
    const report = event.payload as ProgressReport;
    onProgress(report);
  });
}

/**
 * Run detection
 */
export async function process(
  path: string,
  confidenceThreshold: number,
  recursive: boolean,
  onProgress?: (report: ProgressReport) => void
) {
  await invoke("process", {
    path,
    confidenceThreshold,
    recursive,
  });

  if (onProgress) {
    await listen("progress", (event) => {
      const report = event.payload as ProgressReport;
      onProgress(report);
    }).catch((e) => {
      console.error(`Error listening to progress: ${e}`);
    });
  }
}

export type ExportFormat = "json" | "csv";
export type ImageExportFormat = "image-dir";
export type AllExportFormat = ExportFormat | ImageExportFormat;

export async function createExport(
  format: ExportFormat,
  outputPath: string
) {
  return await invoke("export", { format, outputPath });
}

export type FilterCriteriaOption = "Include" | "Intersect" | "Exclude";

export interface FilterCriteria {
  animals: FilterCriteriaOption;
  humans: FilterCriteriaOption;
  vehicles: FilterCriteriaOption;
  empty: FilterCriteriaOption;
}

export function createFilterCriteria(
  animals: FilterCriteriaOption,
  humans: FilterCriteriaOption,
  vehicles: FilterCriteriaOption,
  empty: FilterCriteriaOption
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
  vehicles: boolean
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
  drawCriteria: DrawCriteria
) {
  return invoke("export_image_set", {
    outputPath,
    filterCriteria: filterCriteria,
    drawCriteria: drawCriteria,
  });
}
