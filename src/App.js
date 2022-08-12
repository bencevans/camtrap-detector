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
    listen('progress', (data) => {
      console.log(data)
      setProgress(data.payload)
    }).catch(err => {
      console.log(err);
    }).finally(() => {
      console.log('done');
    })
  }, []);
  

  async function selectDir() {
    const chosenPath = await open({
      directory: true,
    });

    if (chosenPath) {
      setBaseDir(chosenPath);

      invoke('run_detection', {
        baseDir: chosenPath
      })

      setIsProcessing(true);
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
            flexDirection: "row",
            alignContent: "center",
          }}
        >
          <button onClick={selectDir}>Choose Folder</button>
        </div>
      </div>
    );
  }
}

export default App;
