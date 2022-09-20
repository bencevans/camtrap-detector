import { useEffect, useState } from "react";
import "./App.css";
import TauriDropzone from "./TauriDropZone";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";

function process(path, recursive) {
  return invoke("process", {
    path,
    recursive,
  });
}

const formatTypes = [
  {
    id: "ct-csv",
    name: "CamTrap CSV",
    pathType: "file",
    defaultPath: "ct.5.1.0a.csv",
    disabled: false,
    desciption:
      "Comma Separated Values (CSV) file containing a row for each detection from each in the dataset. Recommended for use with Excel, R, etc.",
  },
  {
    id: "ct-json",
    name: "CamTrap JSON",
    pathType: "file",
    defaultPath: "ct.5.1.0a.json",
    disabled: false,
    desciption:
      "JavaScript Object Notation (JSON) file containing a row for each detection from each in the dataset. Recommended for use with Python, R, etc.",
  },
  {
    id: "md",
    name: "MegaDetector Batch JSON",
    pathType: "file",
    defaultPath: "md.5.1.0a.json",

    disabled: false,
    desciption:
      "MegaDetector Batch JSON file, reccomended for use with existing MegaDetector related tooling, e.g. Timelapse, etc.",
  },
  {
    id: "image-dir",
    name: "Image Directory (coming soon)",
    pathType: "dir",
    defaultPath: "",

    disabled: true,
  },
];

function FolderSelect({ onDrop }) {
  const [includeSubfolders, setIncludeSubfolders] = useState(true);

  return (
    <>
      <TauriDropzone
        onDrop={(path) => {
          onDrop(path, includeSubfolders);
        }}
      />
      <label
        style={{
          margin: "auto",
          paddingTop: "10px",
          opacity: 0.5,
        }}
      >
        <input
          type="checkbox"
          checked={includeSubfolders}
          onChange={(e) => setIncludeSubfolders(e.target.checked)}
        />
        <span
          style={{
            paddingLeft: "10px",
            fontSize: "12px",
          }}
        >
          Include Subfolders
        </span>
      </label>
    </>
  );
}

function OutputWizard({ path, includeSubfolders }) {
  useEffect(() => {
    appWindow.setSize(new LogicalSize(700, 600));
  }, []);

  const [formats, setFormats] = useState(
    formatTypes.filter((f) => !f.disabled)
  );

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        height: "100vh",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
          opacity: 0.5,
        }}
      >
        <div>Dataset: {path}</div>
        <div>Include Subfolders: {includeSubfolders ? "Yes" : "No"}</div>
      </div>

      {/* <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
          marginTop: 10,
          opacity: 0.5,
        }}
      >
        <div>
          Confidence Threshold: <input type="number" value="0.1" style={{
            width: 50
          }} />
        </div>
        <div>
          NMS Threshold: <input type="number" value="0.1"  style={{
            width: 50
          }}/>
        </div>
      </div> */}

      {/* <h2>Output Formats</h2> */}
      <p>Please select the output formats you would like:</p>
      <div className="card-container">
        {formats.map((format) => (
          <div
            className="card"
            style={{
              backgroundColor: format.disabled ? "#111" : "#2a2a2a",
            }}
          >
            <div className="card-header">
              <input
                type="checkbox"
                name="output-format"
                id={format.id}
                checked={!format.disabled}
                disabled={format.disabled}
              />
              <label htmlFor={format.id}>{format.name}</label>
            </div>

            {/* <div className="card-body">
              <div className="card-body-item">
                <label htmlFor="output-path">Output Path</label>
                <input
                  type="text"
                  name="output-path"
                  id="output-path"
                  defaultValue={format.defaultPath}
                />

                <button
                  style={{
                    marginLeft: "10px",
                    padding: "5px",
                    borderRadius: "5px",
                    border: "1px solid #ccc",
                  }}
                >
                  Browse
                </button>
              </div>
            </div> */}
          </div>
        ))}
      </div>
    </div>
  );
}

function App() {
  const [path, setPath] = useState(null);
  const [includeSubfolders, setIncludeSubfolders] = useState(null);
  const [isProcessing, setIsProcessing] = useState(true);

  useEffect(() => {
    if (path && includeSubfolders) {
      setIsProcessing(true);
      process(path, includeSubfolders).finally(() => {
        setIsProcessing(false)
      });
    }
  }, [path, includeSubfolders]);

  return (
    <div
      className="App"
      style={{
        display: "flex",
      }}
    >
      {path === null && includeSubfolders === null ? (
        <FolderSelect
          onDrop={(dirPath, recursive) => {
            setPath(dirPath);
            setIncludeSubfolders(recursive);
          }}
        />
      ) : (
        <div>Processing: {JSON.stringify(isProcessing)}</div>
      )}
    </div>
  );
}

export default App;
