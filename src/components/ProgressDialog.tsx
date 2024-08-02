import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { useEffect } from "react";
import { ProgressReport } from "../api";

function friendlyEta(secondsRemaining: number) {
  const hours = Math.trunc(secondsRemaining / 3600);
  const minutes = Math.trunc(secondsRemaining / 60) % 60;
  const seconds = Math.trunc(secondsRemaining / 1) % 60;

  return `${hours}h ${minutes}m ${seconds}s`;
}

export default function ProgressDialog({
  processingStatus,
}: {
  processingStatus: ProgressReport;
}) {
  useEffect(() => {
    appWindow.setSize(new LogicalSize(500, 200)).catch(console.error);
  });

  return (
    <div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          width: "100%",
          marginBottom: 20,
        }}
      >
        <div>{processingStatus.message}</div>
        <div
          style={{
            marginLeft: 5,
            overflow: "hidden",
            textOverflow: "ellipsis",
            whiteSpace: "nowrap",
            // direction: "rtl",
          }}
        >
          {processingStatus.path}
        </div>
      </div>

      <div
        style={{
          backgroundColor: "#2a2a2a",
          padding: 10,
        }}
      >
        <div
          style={{
            width: `${processingStatus.percent}%`,
            height: 10,
            backgroundColor: "#00bfff",
          }}
        />
      </div>

      <div
        style={{
          display: "flex",
          flexDirection: "row",
          width: "100%",
          justifyContent: "space-between",
          marginTop: 20,
        }}
      >
        <div>
          {processingStatus.eta !== null &&
            `ETA ${friendlyEta(processingStatus.eta)}`}
        </div>
        <div>
          {processingStatus.current} / {processingStatus.total} Images
        </div>
      </div>

      <p
        style={{
          textAlign: "right",
        }}
      ></p>
    </div>
  );
}
