function friendlyEta(secondsRemaining) {
  const hours = Math.trunc(secondsRemaining / 3600);
  const minutes = Math.trunc(secondsRemaining / 60) % 60;
  const seconds = Math.trunc(secondsRemaining / 1) % 60;

  return `${hours}h ${minutes}m ${seconds}s`;
}

export default function ProgressDialog({
  processingStatus,
}) {
  return (
    <div>
      <p>{processingStatus.message}</p>

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
        <div>ETA {friendlyEta(processingStatus.eta)}</div>
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
