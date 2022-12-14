import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { FaCog } from "react-icons/fa";

async function isDir(path) {
  return await invoke("is_dir", { path });
}

export default function TauriDropzone({ onDrop, onConfig }) {
  const [isDragActive, setIsDragActive] = useState(false);

  useEffect(() => {
    listen("tauri://file-drop", (event) => {
      let files = event.payload;

      if (files.length > 1) {
        setIsDragActive(false);
        return console.warn("Only one folder at a time is supported");
      }

      if (files.length === 0) {
        setIsDragActive(false);
        return console.warn("No files were dropped");
      }

      let file = files[0];

      isDir(file)
        .then((isDir) => {
          if (isDir) {
            console.log("Dropped (internal)", file);
            onDrop(file);
          } else {
            alert("Only folders are supported");
          }

          setIsDragActive(false);
        })
        .catch((err) => {
          setIsDragActive(false);
        });
    });

    listen("tauri://file-drop-hover", (event) => {
      setIsDragActive(true);
    });

    listen("tauri://file-drop-cancelled", (event) => {
      console.log("canceled", event);
      setIsDragActive(false);
    });
  });

  return (
    <div
      style={{
        borderWidth: 4,
        borderRadius: 8,
        borderColor: isDragActive ? "#2196f3" : "#ccc",
        borderStyle: "dashed",
        padding: 20,
        textAlign: "center",
        height: "100%",
        transition: "border 0.2s ease-in-out",
        display: "flex",
        flexDirection: "column",
        justifyContent: "space-around",
        alignItems: "center",
        position: "relative",
      }}
    >
      <div>
        <p style={{ margin: "auto" }}>
          Drag 'n' drop your Camera Trap Image Folder
        </p>
        <button
          style={{
            margin: "auto",
            marginTop: "20px",
          }}
          onClick={() => {
            open({
              directory: true,
              multiple: false,
            })
              .then((res) => {
                if (res !== null) {
                  onDrop(res);
                }
              })
              .catch((err) => {
                console.log(err);
              });
          }}
        >
          Select Folder
        </button>
      </div>
      {onConfig && (
        <button
          onClick={onConfig}
          style={{
            position: "absolute",
            bottom: "10px",
            right: "10px",
            border: "none",
            background: "none",
            color: "white",
            opacity: 0.5,
            fontSize: "12px",
            padding: "6px",
            cursor: "pointer",
          }}
        >
          <FaCog />
        </button>
      )}
    </div>
  );
}
