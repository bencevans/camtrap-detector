import { useEffect, useState } from "react";
import TauriDropzone from "./TauriDropZone";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { LogicalSize } from "@tauri-apps/api/window";
import ConfigDialog, { Config } from "./ConfigDialog";

export default function FolderSelectDialog({
  onDrop,
  onConfig,
  config,
}: {
  onDrop: (path: string, includeSubfolders: boolean) => void;
  onConfig: (config: Config) => void;
  config: Config;
}) {
  const [includeSubfolders, setIncludeSubfolders] = useState(true);
  const [showConfig, setShowConfig] = useState(false);

  useEffect(() => {
    getCurrentWindow().setSize(new LogicalSize(500, 300)).catch(console.error);
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
