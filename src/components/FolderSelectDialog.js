import { useEffect, useState } from "react";
import TauriDropzone from "./TauriDropZone";
import { appWindow } from "@tauri-apps/api/window";
import { LogicalSize } from "@tauri-apps/api/window";

export default function FolderSelectDialog({ onDrop }) {
  const [includeSubfolders, setIncludeSubfolders] = useState(true);

  useEffect(() => {
    appWindow.setSize(new LogicalSize(500, 300));
  });

  return (
    <>
      <TauriDropzone
        onDrop={(path) => {
          onDrop(path, includeSubfolders);
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
