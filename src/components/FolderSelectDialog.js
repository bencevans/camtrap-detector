import { useState } from "react";
import TauriDropzone from "./TauriDropZone";



export default function FolderSelectDialog({ onDrop }) {
  const [includeSubfolders, setIncludeSubfolders] = useState(true);

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