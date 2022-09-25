import { appWindow } from "@tauri-apps/api/window";
import { LogicalSize } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";
import {
  createDrawCriteria,
  createExport,
  createFilterCriteria,
  exportImageSet,
} from "../api";
import "./ExportDialog.css";

const formatTypes = [
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
      "Directory containing images from the dataset, optionaly including or exclusing particular classes and optionally with detections drawn on.",
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

export default function ExportDialog() {
  useEffect(() => {
    appWindow.setSize(new LogicalSize(600, 600));
  });

  const [imageExportAnimalFilter, setImageExportAnimalFilter] =
    useState("Include");
  const [imageExportHumanFilter, setImageExportHumanFilter] = useState("Union");
  const [imageExportVehicleFilter, setImageExportVehicleFilter] =
    useState("Union");
  const [imageExportEmptyFilter, setImageExportEmptyFilter] = useState("Union");

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
                    <th>Union</th>
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
                        onChange={(e) =>
                          setImageExportAnimalFilter(e.target.value)
                        }
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="animals"
                        value="Union"
                        checked={imageExportAnimalFilter === "Union"}
                        onChange={(e) =>
                          setImageExportAnimalFilter(e.target.value)
                        }
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="animals"
                        value="Exclude"
                        disabled={imageExportAnimalFilter === "Exclude"}
                        checked={imageExportAnimalFilter === "Exclude"}
                        onChange={(e) =>
                          setImageExportAnimalFilter(e.target.value)
                        }
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
                        onChange={(e) =>
                          setImageExportHumanFilter(e.target.value)
                        }
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="humans"
                        value="Union"
                        checked={imageExportHumanFilter === "Union"}
                        onChange={(e) =>
                          setImageExportHumanFilter(e.target.value)
                        }
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="humans"
                        value="Exclude"
                        checked={imageExportHumanFilter === "Exclude"}
                        onChange={(e) =>
                          setImageExportHumanFilter(e.target.value)
                        }
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
                        onChange={(e) =>
                          setImageExportVehicleFilter(e.target.value)
                        }
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="vehicles"
                        value="Union"
                        checked={imageExportVehicleFilter === "Union"}
                        onChange={(e) =>
                          setImageExportVehicleFilter(e.target.value)
                        }
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="vehicles"
                        value="Exclude"
                        checked={imageExportVehicleFilter === "Exclude"}
                        onChange={(e) =>
                          setImageExportVehicleFilter(e.target.value)
                        }
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
                        onChange={(e) =>
                          setImageExportEmptyFilter(e.target.value)
                        }
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="empty"
                        value="Union"
                        checked={imageExportEmptyFilter === "Union"}
                        onChange={(e) =>
                          setImageExportEmptyFilter(e.target.value)
                        }
                      />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="empty"
                        value="Exclude"
                        checked={imageExportEmptyFilter === "Exclude"}
                        onChange={(e) =>
                          setImageExportEmptyFilter(e.target.value)
                        }
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
            <button
              onClick={() => {
                if (format.id === "image-dir") {
                  exportImageSet(
                    createFilterCriteria(
                      imageExportAnimalFilter,
                      imageExportHumanFilter,
                      imageExportVehicleFilter,
                      imageExportEmptyFilter
                    ),
                    createDrawCriteria(true, true, true)
                  );
                } else {
                  createExport(format.id);
                }
              }}
            >
              Export
            </button>
          </div>
        </div>
      ))}
    </div>
  );
}
