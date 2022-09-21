import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import { LogicalSize } from "@tauri-apps/api/window";
import { useEffect } from "react";
import "./ExportDialog.css";

function createExport(format) {
  return invoke("export", { format });
}

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
                        value="yes"
                        checked={true}
                      />
                    </td>
                    <td>
                      <input type="radio" name="animals" value="no" />
                    </td>
                  </tr>
                  <tr>
                    <th>Humans</th>
                    <td>
                      <input
                        type="radio"
                        name="humans"
                        value="yes"
                        checked={true}
                      />
                    </td>
                    <td>
                      <input type="radio" name="humans" value="no" />
                    </td>
                  </tr>
                  <tr>
                    <th>Vehicles</th>
                    <td>
                      <input
                        type="radio"
                        name="vehicles"
                        value="yes"
                        checked={true}
                      />
                    </td>
                    <td>
                      <input type="radio" name="vehicles" value="no" />
                    </td>
                  </tr>
                  <tr>
                    <th>Empty</th>
                    <td>
                      <input type="radio" name="empty" value="yes" />
                    </td>
                    <td>
                      <input
                        type="radio"
                        name="empty"
                        value="no"
                        checked={true}
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
                createExport(format.id);
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
