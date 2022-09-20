import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import { readDir } from "@tauri-apps/api/fs";

export default function TauriDropzone() {
  const [files, setFiles] = useState([]);
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

      readDir(file)
        .then((files) => {
          console.log(files);
          setIsDragActive(false);
        })
        .catch((err) => {
          console.error(err);
          setIsDragActive(false);
        });
    });

    listen("tauri://file-drop-hover", (event) => {
      console.log(event);
      setIsDragActive(true);
    });

    listen("tauri://file-drop-cancelled", (event) => {
      console.log("canceled", event);
      setIsDragActive(false);
    });
  }, []);

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
            })
              .then((res) => {
                console.log(res);
                setFiles(res);
              })
              .catch((err) => {
                console.log(err);
              });
          }}
        >
          Select Folder
        </button>
      </div>
    </div>
  );
}
