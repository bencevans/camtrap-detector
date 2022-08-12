import "./App.css";
import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [baseDir, setBaseDir] = useState(null);
  const [isProcessing, setIsProcessing] = useState(false);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    listen("progress", (data) => {
      console.log(data);
      setProgress(data.payload);
    })
      .catch((err) => {
        console.log(err);
      })
      .finally(() => {
        console.log("done");
      });
  }, []);

  async function selectDir() {
    const chosenPath = await open({
      directory: true,
    });

    if (chosenPath) {
      setBaseDir(chosenPath);

      // invoke('run_detection', {
      //   baseDir: chosenPath,
      //   relativePaths: false,
      //   outputJson: chosenPath + '/output2.json',
      // })

      // setIsProcessing(true);
    }
  }

  if (isProcessing) {
    return <div>Processing... {progress}</div>;
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
          <div style={{
            backgroundColor: "#010101",
            width: "100%",
            padding: "10px",
          }}>
            <b>Images</b>
          </div>
          <div style={{ padding: 10 }}>
            <button onClick={selectDir}>Choose Folder</button> {baseDir}
          </div>

          {/* <div style={{
            backgroundColor: "#010101",
            width: "100%",
            padding: "10px",
          }}>
            <b>Model</b>
          </div>
          <div style={{ padding: 10 }}>
            <p>MegaDetector v5.0 ONNX Converted</p>
          </div> */}

          <div style={{
            backgroundColor: "#010101",
            width: "100%",
            padding: "10px",
          }}>
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
              <li>
                <input
                  id="output-json-enabled"
                  type="checkbox"
                  checked={true}
                />{" "}
                <label for="output-json-enabled">JSON Batch File</label>
                <div>
                  <button>Change Location</button>
                  {baseDir + "/output.json"}
                </div>
              </li>
              <li>
                <input id="output-csv-enabled" type="checkbox" />{" "}
                <label for="output-csv-enabled">CSV File</label>
                <div>
                  <button>Change Location</button>
                  {baseDir + "/output.csv"}
                </div>
              </li>
              <li>
                <input id="output-filtereddir-enabled" type="checkbox" />{" "}
                <label for="output-filtereddir-enabled">Filtered Images</label>
                <div>
                  <button>Change Location</button>
                  {baseDir + "/../filtered"}
                </div>
              </li>
            </ul>
          </div>
        </div>
      </div>
    );
  }
}

export default App;
