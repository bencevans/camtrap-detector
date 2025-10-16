import { open, save } from "@tauri-apps/plugin-dialog";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { LogicalSize } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";
import { PulseLoader } from "react-spinners";
import {
  AllExportFormat,
  createDrawCriteria,
  createExport,
  createFilterCriteria,
  exportImageSet,
  FilterCriteriaOption,
} from "../api";
import "./ExportDialog.css";

interface Format {
  id: AllExportFormat;
  name: string;
  pathType: "file" | "dir";
  defaultPath: string;
  disabled: boolean;
  description: string;
  options?: {
    id: string;
    name: string;
    options: {
      id: string;
      name: string;
      value: string;
    }[];
  }[];
}

const formatTypes: Format[] = [
  {
    id: "csv",
    name: "CamTrap CSV",
    pathType: "file",
    defaultPath: "ct.5.1.0a.csv",
    disabled: false,
    description:
      "Comma Separated Values (CSV) file containing a row for each detection from each in the dataset. Recommended for use with Excel, R, etc.",
  },
  {
    id: "json",
    name: "CamTrap JSON",
    pathType: "file",
    defaultPath: "ct.5.1.0a.json",
    disabled: false,
    description:
      "JavaScript Object Notation (JSON) file containing a row for each detection from each in the dataset. Recommended for use with Python, R, etc.",
  },
  {
    id: "image-dir",
    name: "Image Directory",
    description:
      "Directory containing images from the dataset, optionally including or excluding particular classes with detections drawn on.",
    pathType: "dir",
    defaultPath: "",
    options: [
      {
        id: "humans",
        name: "Humans",
        options: [
          {
            id: "human-yes",
            name: "Yes",
            value: "yes",
          },
          {
            id: "human-no",
            name: "No",
            value: "no",
          },
        ],
      },
    ],
    disabled: false,
  },
];

export default function ExportDialog({ onReset }: { onReset: () => void }) {
  useEffect(() => {
    getCurrentWindow().setSize(new LogicalSize(600, 650)).catch(console.error);
  }, []);

  const [imageExportAnimalFilter, setImageExportAnimalFilter] = useState(
    "Include" as FilterCriteriaOption
  );
  const [imageExportHumanFilter, setImageExportHumanFilter] = useState(
    "Intersect" as FilterCriteriaOption
  );
  const [imageExportVehicleFilter, setImageExportVehicleFilter] = useState(
    "Intersect" as FilterCriteriaOption
  );
  const [imageExportEmptyFilter, setImageExportEmptyFilter] = useState(
    "Intersect" as FilterCriteriaOption
  );

  const [exportInProgress, setExportInProgress] = useState([] as string[]);
  const [exportError, setExportError] = useState<string | null>(null);
  const [exportSuccess, setExportSuccess] = useState<string | null>(null);

  // Helper to handle export errors
  const handleExportError = (formatName: string, error: unknown) => {
    let message = "";
    if (error instanceof Error) {
      message = error.message;
    } else if (typeof error === "string") {
      message = error;
    } else {
      message = JSON.stringify(error);
    }
    setExportError(`Failed to export ${formatName}: ${message}`);
    setTimeout(() => setExportError(null), 6000);
  };

  // Helper to handle export success
  const handleExportSuccess = (formatName: string) => {
    setExportSuccess(`${formatName} export completed successfully.`);
    setTimeout(() => setExportSuccess(null), 4000);
  };

  // Helper to start export
  const startExport = (format: Format) => {
    setExportError(null);
    setExportSuccess(null);
    if (exportInProgress.includes(format.id)) return;
    setExportInProgress((prev) => [...prev, format.id]);
    void (async () => {
      try {
        if (format.id === "image-dir") {
          const outputPath = await open({ directory: true });
          if (!outputPath || Array.isArray(outputPath)) {
            setExportInProgress((prev) => prev.filter((id) => id !== format.id));
            return;
          }
          await exportImageSet(
            outputPath,
            createFilterCriteria(
              imageExportAnimalFilter,
              imageExportHumanFilter,
              imageExportVehicleFilter,
              imageExportEmptyFilter
            ),
            createDrawCriteria(true, true, true)
          );
        } else {
          const defaultFileName =
            format.id === "json" ? "ct.0.1.0.json" : "ct.0.1.0.csv";
          const outputPath = await save({ defaultPath: defaultFileName });
          if (!outputPath || Array.isArray(outputPath)) {
            setExportInProgress((prev) => prev.filter((id) => id !== format.id));
            return;
          }
          await createExport(format.id, outputPath);
        }
        handleExportSuccess(format.name);
      } catch (error) {
        handleExportError(format.name, error);
      } finally {
        setExportInProgress((prev) => prev.filter((id) => id !== format.id));
      }
    })();
  };

  return (
    <div>
      {exportError && (
        <div
          className="export-error"
          style={{
            color: "#ff4d4f",
            marginBottom: 10,
            background: "#2a1a1a",
            padding: 8,
            borderRadius: 4,
          }}
        >
          {exportError}
        </div>
      )}
      {exportSuccess && (
        <div
          className="export-success"
          style={{
            color: "#00e676",
            marginBottom: 10,
            background: "#1a2a1a",
            padding: 8,
            borderRadius: 4,
          }}
        >
          {exportSuccess}
        </div>
      )}
      {formatTypes.map((format) => (
        <div
          key={format.name}
          style={{
            display: "flex",
            flexDirection: "row",
            marginBottom: 20,
            backgroundColor: "#2a2a2a",
            padding: 10,
            borderRadius: 5,
          }}
        >
          <div>
            <h3 style={{ color: "#00bfff", margin: 0 }}>{format.name}</h3>
            <p style={{ fontSize: 12 }}>{format.description}</p>
            {format.id === "image-dir" && (
              <table style={{ width: "100%" }}>
                <thead>
                  <tr>
                    <th></th>
                    <th>Include</th>
                    <th>Intersect</th>
                    <th>Exclude</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <th>Animals</th>
                    <td>
                      <input
                        type="radio"
                        name="animals"
                        value="Include"
                        checked={imageExportAnimalFilter === "Include"}
                        onChange={() => setImageExportAnimalFilter("Include")}
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="animals"
                        value="Intersect"
                        checked={imageExportAnimalFilter === "Intersect"}
                        onChange={() => setImageExportAnimalFilter("Intersect")}
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="animals"
                        value="Exclude"
                        checked={imageExportAnimalFilter === "Exclude"}
                        onChange={() => setImageExportAnimalFilter("Exclude")}
                      />
                    </td>
                  </tr>
                  <tr>
                    <th>Humans</th>
                    <td>
                      <input
                        type="radio"
                        name="humans"
                        value="Include"
                        checked={imageExportHumanFilter === "Include"}
                        onChange={() => setImageExportHumanFilter("Include")}
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="humans"
                        value="Intersect"
                        checked={imageExportHumanFilter === "Intersect"}
                        onChange={() => setImageExportHumanFilter("Intersect")}
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="humans"
                        value="Exclude"
                        checked={imageExportHumanFilter === "Exclude"}
                        onChange={() => setImageExportHumanFilter("Exclude")}
                      />
                    </td>
                  </tr>
                  <tr>
                    <th>Vehicles</th>
                    <td>
                      <input
                        type="radio"
                        name="vehicles"
                        value="Include"
                        checked={imageExportVehicleFilter === "Include"}
                        onChange={() => setImageExportVehicleFilter("Include")}
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="vehicles"
                        value="Intersect"
                        checked={imageExportVehicleFilter === "Intersect"}
                        onChange={() => setImageExportVehicleFilter("Intersect")}
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="vehicles"
                        value="Exclude"
                        checked={imageExportVehicleFilter === "Exclude"}
                        onChange={() => setImageExportVehicleFilter("Exclude")}
                      />
                    </td>
                  </tr>
                  <tr>
                    <th>Empty</th>
                    <td>
                      <input
                        type="radio"
                        name="empty"
                        value="Include"
                        checked={imageExportEmptyFilter === "Include"}
                        onChange={() => setImageExportEmptyFilter("Include")}
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="empty"
                        value="Intersect"
                        checked={imageExportEmptyFilter === "Intersect"}
                        onChange={() => setImageExportEmptyFilter("Intersect")}
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="empty"
                        value="Exclude"
                        checked={imageExportEmptyFilter === "Exclude"}
                        onChange={() => setImageExportEmptyFilter("Exclude")}
                      />
                    </td>
                  </tr>
                </tbody>
              </table>
            )}
          </div>
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              justifyContent: "center",
              paddingLeft: 10,
            }}
          >
            {exportInProgress.includes(format.id) ? (
              <PulseLoader size={11} color={"#00bfff"} />
            ) : (
              <button
                disabled={format.disabled}
                onClick={() => startExport(format)}
              >
                Export
              </button>
            )}
          </div>
        </div>
      ))}
      {/* New Run Button */}
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
        }}
      >
        <button
          style={{
            padding: 10,
            borderRadius: 5,
            backgroundColor: "#000",
            color: "#fff",
            border: "none",
          }}
          onClick={() => {
            onReset();
          }}
        >
          New Run
        </button>
      </div>
    </div>
  );
}
