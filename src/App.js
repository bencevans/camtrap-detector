import "./App.css";
import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [baseDir, setBaseDir] = useState(null);
  const [csvOuputActive, setCsvOuputActive] = useState(false);
  const [filteredAnimalsOutputActive, setFilteredAnimalsOutputActive] =
    useState(false);

  const [isProcessing, setIsProcessing] = useState(false);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    listen("progress", (data) => {
      setProgress(data.payload);
    })
      .catch((err) => {
        setProgress(err);
      })
  }, []);

  async function selectDir() {
    const chosenPath = await open({
      directory: true,
    });

    if (chosenPath) {
      setBaseDir(chosenPath);
    }
  }

  if (isProcessing) {
    return <div style={{
      width: "100%",
      height: "100%",
      display: "flex",
      justifyContent: "center",
      flexDirection: "column",
      alignItems: "center",
      // space-between",
      
    }}>Processing... {progress}</div>;
  } else {
    return (
      <div className="App">
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            alignContent: "center",
            width: "100%",
          }}
        >
          <div
            style={{
              backgroundColor: "#010101",
              width: "100%",
              padding: "10px",
            }}
          >
            <b>Images</b>
          </div>
          <div style={{ padding: 10 }}>
            <button onClick={selectDir}>Choose Folder</button> {baseDir}
          </div>

          <div
            style={{
              backgroundColor: "#010101",
              width: "100%",
              padding: "10px",
            }}
          >
            <b>Outputs</b>
          </div>
          <div style={{ padding: 10 }}>
            <ul
              style={{
                listStyleType: "none",
                padding: 0,
                margin: 0,
              }}
            >
              <li style={{ paddingBottom: 10 }}>
                <input
                  id="output-json-enabled"
                  type="checkbox"
                  checked={true}
                  disabled={true}
                  readOnly={true}
                />{" "}
                <label htmlFor="output-json-enabled">JSON File</label>
                <div
                  style={{
                    paddingLeft: 20,
                    fontFamily: "monospace",
                    fontSize: 12,
                  }}
                >
                  {/* <button>Change Location</button> */}
                  {baseDir + "/camtrap-detector.0.1.0.json"}
                </div>
              </li>
              <li style={{ paddingBottom: 10 }}>
                <input
                  id="output-csv-enabled"
                  type="checkbox"
                  checked={csvOuputActive}
                  onClick={() => {
                    setCsvOuputActive(!csvOuputActive);
                  }}
                />{" "}
                <label htmlFor="output-csv-enabled">CSV File</label>
                <div
                  style={{
                    paddingLeft: 20,
                    fontFamily: "monospace",
                    fontSize: 12,
                  }}
                >
                  {/* <button>Change Location</button> */}
                  {baseDir + "/camtrap-detector.0.1.0.csv"}
                </div>
              </li>
              <li>
                <input
                  id="output-filtereddir-enabled"
                  type="checkbox"
                  checked={filteredAnimalsOutputActive}
                  onClick={() => {
                    setFilteredAnimalsOutputActive(
                      !filteredAnimalsOutputActive
                    );
                  }}
                />{" "}
                <label htmlFor="output-filtereddir-enabled">Filtered Images</label>
                <div
                  style={{
                    paddingLeft: 20,
                    fontFamily: "monospace",
                    fontSize: 12,
                  }}
                >
                  {/* <button>Change Location</button> */}
                  {baseDir + "/../filtered"}
                </div>
              </li>
            </ul>
          </div>
          <div
            style={{
              padding: 10,
            }}
          >
            <button
              style={{ width: "100%" }}
              disabled={!baseDir}
              onClick={() => {
                setIsProcessing(true);
                invoke("run_detection", {
                  baseDir: baseDir,
                  relativePaths: false,
                  outputJson: baseDir + "/camtrap-detector.0.1.0.json",
                  outputCsv: baseDir + "/camtrap-detector.0.1.0.csv",
                  outputAnimalsFolder: baseDir + "/../filtered",
                });
              }}
            >
              Begin Processing
            </button>
          </div>
        </div>
      </div>
    );
  }
}

export default App;
