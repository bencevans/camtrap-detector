import { useEffect, useState } from "react";
import "./App.css";
import TauriDropzone from "./TauriDropZone";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

function process(path, recursive) {
  return invoke("process", {
    path,
    recursive,
  });
}

function friendlyEta(secondsRemaining) {
  const hours = Math.trunc(secondsRemaining / 3600);
  const minutes = Math.trunc(secondsRemaining / 60) % 60;
  const seconds = Math.trunc(secondsRemaining / 1) % 60;

  return `${hours}h ${minutes}m ${seconds}s`;
}

// const formatTypes = [
//   {
//     id: "ct-csv",
//     name: "CamTrap CSV",
//     pathType: "file",
//     defaultPath: "ct.5.1.0a.csv",
//     disabled: false,
//     desciption:
//       "Comma Separated Values (CSV) file containing a row for each detection from each in the dataset. Recommended for use with Excel, R, etc.",
//   },
//   {
//     id: "ct-json",
//     name: "CamTrap JSON",
//     pathType: "file",
//     defaultPath: "ct.5.1.0a.json",
//     disabled: false,
//     desciption:
//       "JavaScript Object Notation (JSON) file containing a row for each detection from each in the dataset. Recommended for use with Python, R, etc.",
//   },
//   {
//     id: "md",
//     name: "MegaDetector Batch JSON",
//     pathType: "file",
//     defaultPath: "md.5.1.0a.json",

//     disabled: false,
//     desciption:
//       "MegaDetector Batch JSON file, reccomended for use with existing MegaDetector related tooling, e.g. Timelapse, etc.",
//   },
//   {
//     id: "image-dir",
//     name: "Image Directory (coming soon)",
//     pathType: "dir",
//     defaultPath: "",

//     disabled: true,
//   },
// ];

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

function App() {
  const [path, setPath] = useState(null);
  const [includeSubfolders, setIncludeSubfolders] = useState(null);
  const [processingStatus, setProcessingStatus] = useState(null);

  useEffect(() => {
    listen("progress", (event) => {
      setProcessingStatus(event.payload);
    });
  }, []);

  useEffect(() => {
    if (path && includeSubfolders) {
      process(path, includeSubfolders)
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
        <>
          {processingStatus ? (
            <div>
              <p>{processingStatus.message}</p>

              <div
                style={{
                  backgroundColor: "#2a2a2a",
                  padding: 10,
                }}
              >
                <div
                  style={{
                    width: `${processingStatus.percent}%`,
                    height: 10,
                    backgroundColor: "#00bfff",
                  }}
                />
              </div>

              <div
                style={{
                  display: "flex",
                  flexDirection: "row",
                  width: "100%",
                  justifyContent: "space-between",
                  marginTop: 20,
                }}
              >
                <div>ETA {friendlyEta(processingStatus.eta)}</div>
                <div>
                  {processingStatus.current} / {processingStatus.total} Images
                </div>
              </div>

              <p
                style={{
                  textAlign: "right",
                }}
              ></p>
            </div>
          ) : null}
        </>
      )}
    </div>
  );
}

export default App;
