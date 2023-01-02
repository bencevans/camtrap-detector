import { useEffect, useState } from "react";
import TauriDropzone from "./TauriDropZone";
import { appWindow } from "@tauri-apps/api/window";
import { LogicalSize } from "@tauri-apps/api/window";
import ConfigDialog from "./ConfigDialog";

export default function FolderSelectDialog({ onDrop, onConfig, config }) {
  const [includeSubfolders, setIncludeSubfolders] = useState(true);
  const [showConfig, setShowConfig] = useState(false);

  useEffect(() => {
    appWindow.setSize(new LogicalSize(500, 300));
  });

  if (showConfig) {
    return (
      <ConfigDialog
        onClose={() => setShowConfig(false)}
        onConfig={onConfig}
        config={config}
      />
    );
  }

  return (
    <>
      <TauriDropzone
        onDrop={(path) => {
          onDrop(path, includeSubfolders);
        }}
        onConfig={() => {
          setShowConfig(true);
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
