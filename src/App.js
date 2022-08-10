import "./App.css";
import { open } from "@tauri-apps/api/dialog";
import { useState } from "react";
import { invoke } from "@tauri-apps/api";

function App() {
  const [baseDir, setBaseDir] = useState(null);
  const [isProcessing, setIsProcessing] = useState(false);

  async function selectDir() {
    const chosenPath = await open({
      directory: true,
    });

    if (chosenPath) {
      setBaseDir(chosenPath);

      invoke('run_detection', {
        baseDir: chosenPath
        
      })
    }
  }

  if (isProcessing) {
    return <div>Processing...</div>;
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
