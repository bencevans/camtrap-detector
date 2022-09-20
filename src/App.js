import "./App.css";
import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { exit } from "@tauri-apps/api/process";
import { useDropzone } from "react-dropzone";

// Styles

import TauriDropzone from "./TauriDropZone";

function App() {
  return <div className="App" style={{
    display: "flex",
  }}>
    <TauriDropzone/>
    <label style={{
      margin: "auto",
      paddingTop: "10px",
      opacity: 0.5,
    }}>
      <input type="checkbox" checked={true}/>
      <span style={{
        paddingLeft: "10px",
        fontSize: "12px",
      }}>Include Subfolders</span>
    </label>
  </div>;
}

export default App;
