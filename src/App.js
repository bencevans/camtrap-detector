import { useEffect, useState } from "react";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import FolderSelectDialog from "./components/FolderSelectDialog";
import ProgressDialog from "./components/ProgressDialog";
import ExportDialog from "./components/ExportDialog";
import { process } from "./api";
import { invoke } from "@tauri-apps/api";

function App() {
  const [path, setPath] = useState(null);
  const [includeSubfolders, setIncludeSubfolders] = useState(null);
  const [processingStatus, setProcessingStatus] = useState(null);
  const [confidenceThreshold, setConfidenceThreshold] = useState(0.1);

  // This is a hack to get the app to show up after the webview is loaded
  useEffect(() => {
    invoke("showup");
  }, []);

  useEffect(() => {
    listen("progress", (event) => {
      setProcessingStatus(event.payload);
    });
  }, []);

  useEffect(() => {
    if (path && includeSubfolders) {
      process(path, confidenceThreshold, includeSubfolders);
    }
  }, [path, confidenceThreshold, includeSubfolders]);

  const resetApp = () => {
    setPath(null);
    setIncludeSubfolders(null);
    setProcessingStatus(null);
  };

  return (
    <div
      className="App"
      style={{
        display: "flex",
      }}
    >
      {processingStatus == null ? (
        <FolderSelectDialog
          config={{
            confidenceThreshold: confidenceThreshold,
          }}
          onDrop={(dirPath, recursive) => {
            setPath(dirPath);
            setIncludeSubfolders(recursive);
          }}
          onConfig={(config) => {
            setConfidenceThreshold(config.confidenceThreshold);
          }}
        />
      ) : (
        <>
          {processingStatus.percent < 100 ? (
            <ProgressDialog processingStatus={processingStatus} />
          ) : (
            <ExportDialog onReset={resetApp} />
          )}
        </>
      )}
    </div>
  );
}

export default App;
