import { open } from "@tauri-apps/plugin-dialog";
import { useEffect, useState } from "react";
import { FaCog } from "react-icons/fa";
import { isDir } from "../api";
import { getCurrentWebview } from "@tauri-apps/api/webview";

export default function TauriDropzone({
  onDrop,
  onConfig,
}: {
  onDrop: (path: string) => void;
  onConfig?: () => void;
}) {
  const [isDragActive, setIsDragActive] = useState(false);

  useEffect(() => {
    const setupDragDropListener = async () => {
      const unlisten = await getCurrentWebview().onDragDropEvent((event) => {
        if (event.payload.type === "over") {
          setIsDragActive(true);
        } else if (event.payload.type === "drop") {
          if (event.payload.paths.length > 1) {
            setIsDragActive(false);
            return console.warn("Only one folder at a time is supported");
          }

          if (event.payload.paths.length === 0) {
            setIsDragActive(false);
            return console.warn("No files were dropped");
          }

          const file = event.payload.paths[0];

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
            .catch(() => {
              setIsDragActive(false);
            });
        } else {
          setIsDragActive(false);
        }
      });

      return unlisten;
    };

    const cleanupPromise = setupDragDropListener();

    return () => {
      cleanupPromise
        .then((unlisten) => unlisten())
        .catch((err) => console.error("Error during cleanup:", err));
    };
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
