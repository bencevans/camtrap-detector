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
  ExportFormat,
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
  desciption: string;
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
    desciption:
      "Comma Separated Values (CSV) file containing a row for each detection from each in the dataset. Recommended for use with Excel, R, etc.",
  },
  {
    id: "json",
    name: "CamTrap JSON",
    pathType: "file",
    defaultPath: "ct.5.1.0a.json",
    disabled: false,
    desciption:
      "JavaScript Object Notation (JSON) file containing a row for each detection from each in the dataset. Recommended for use with Python, R, etc.",
  },
  {
    id: "image-dir",
    name: "Image Directory",
    desciption:
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
    disabled: true,
  },
];

export default function ExportDialog({ onReset }: { onReset: () => void }) {
  useEffect(() => {
    getCurrentWindow().setSize(new LogicalSize(600, 650)).catch(console.error);
  });

  const [imageExportAnimalFilter, setImageExportAnimalFilter] = useState(
    "Include" as FilterCriteriaOption,
  );
  const [imageExportHumanFilter, setImageExportHumanFilter] = useState(
    "Intersect" as FilterCriteriaOption,
  );
  const [imageExportVehicleFilter, setImageExportVehicleFilter] = useState(
    "Intersect" as FilterCriteriaOption,
  );
  const [imageExportEmptyFilter, setImageExportEmptyFilter] = useState(
    "Intersect" as FilterCriteriaOption,
  );

  const [exportInProgress, setExportInProgress] = useState([] as string[]);

  return (
    <div>
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
            <h3
              style={{
                color: "#00bfff",
                margin: 0,
              }}
            >
              {format.name}
            </h3>
            <p
              style={{
                fontSize: 12,
              }}
            >
              {format.desciption}
            </p>

            {format.id === "image-dir" && (
              <table
                style={{
                  width: "100%",
                }}
              >
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
                        disabled={imageExportAnimalFilter === "Exclude"}
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
                        onChange={() =>
                          setImageExportVehicleFilter("Intersect")
                        }
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
                onClick={() => {
                  if (format.id === "image-dir") {
                    open({
                      directory: true,
                    })
                      .then((outputPath) => {
                        if (outputPath === null || Array.isArray(outputPath)) {
                          return;
                        }

                        exportInProgress.push(format.id);
                        setExportInProgress([...exportInProgress]);
                        console.log("Exporting", format.id);

                        exportImageSet(
                          outputPath,
                          createFilterCriteria(
                            imageExportAnimalFilter,
                            imageExportHumanFilter,
                            imageExportVehicleFilter,
                            imageExportEmptyFilter,
                          ),
                          createDrawCriteria(true, true, true),
                        )
                          .then(() => {
                            console.log("Exported", format.id);
                            exportInProgress.splice(
                              exportInProgress.indexOf(format.id),
                              1,
                            );
                            setExportInProgress([...exportInProgress]);
                          })
                          .catch(console.error);
                      })
                      .catch(console.error);
                  } else {
                    const defaultFileName =
                      format.id === "json" ? "ct.0.1.0.json" : "ct.0.1.0.csv";

                    save({
                      defaultPath: defaultFileName,
                    })
                      .then((outputPath) => {
                        if (outputPath === null || Array.isArray(outputPath)) {
                          return;
                        }

                        exportInProgress.push(format.id);
                        setExportInProgress([...exportInProgress]);
                        console.log("Exporting", format.id);

                        createExport(format.id as ExportFormat, outputPath)
                          .then(() => {
                            console.log("Exported", format.id);
                            exportInProgress.splice(
                              exportInProgress.indexOf(format.id),
                              1,
                            );
                            setExportInProgress([...exportInProgress]);
                          })
                          .catch(console.error);
                      })
                      .catch(console.error);
                  }
                }}
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
