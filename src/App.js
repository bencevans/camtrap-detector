import { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import FolderSelectDialog from "./FolderSelectDialog";
import ProgressDialog from "./ProgressDialog";
import ExportDialog from "./ExportDialog";

function process(path, recursive) {
  return invoke("process", {
    path,
    recursive,
  });
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
      process(path, includeSubfolders);
    }
  }, [path, includeSubfolders]);

  return (
    <div
      className="App"
      style={{
        display: "flex",
      }}
    >
      {(processingStatus == null ? (
        <FolderSelectDialog
          onDrop={(dirPath, recursive) => {
            setPath(dirPath);
            setIncludeSubfolders(recursive);
          }}
        />
      ) : (
        <>
          {processingStatus.percent < 100 ? (
            <ProgressDialog processingStatus={processingStatus} />
          ) : (
            <ExportDialog />
          )}
        </>
      ))}
    </div>
  );
}

export default App;
