import { useEffect, useState } from "react";
import "./App.css";
import FolderSelectDialog from "./components/FolderSelectDialog";
import ProgressDialog from "./components/ProgressDialog";
import ExportDialog from "./components/ExportDialog";
import { listenProgress, process, ProgressReport, showWindow } from "./api";

function App() {
  const [path, setPath] = useState(null as null | string);
  const [includeSubfolders, setIncludeSubfolders] = useState(
    null as null | boolean,
  );
  const [processingStatus, setProcessingStatus] = useState(
    null as null | ProgressReport,
  );
  const [confidenceThreshold, setConfidenceThreshold] = useState(0.1 as number);

  // Initial Load
  useEffect(() => {
    const unlistenFunctions: (() => void)[] = [];

    // Show the window, by default it is hidden
    // This is done to avoid showing the window before the interface is ready
    showWindow().catch((e) => {
      console.error(`Error showing window: ${e}`);
    });

    // Listen to progress events
    listenProgress((progressReport) => {
      setProcessingStatus(progressReport);
    })
      .then((unlistenFunc) => {
        unlistenFunctions.push(unlistenFunc);
      })
      .catch((e) => {
        console.error(`Error listening to progress: ${e}`);
      });

    return () => {
      unlistenFunctions.forEach((unlisten) => unlisten());
    };
  }, []);

  useEffect(() => {
    if (path && includeSubfolders) {
      process(path, confidenceThreshold, includeSubfolders).catch((e) => {
        console.error(`Error processing: ${e}`);
      });
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
