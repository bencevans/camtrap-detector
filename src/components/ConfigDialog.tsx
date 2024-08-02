import { FaHome } from "react-icons/fa";

const inputStyle = {
  width: "100%",
};

export interface Config {
  confidenceThreshold: number;
}

export default function ConfigDialog({
  onClose,
  onConfig,
  config,
}: {
  onClose: () => void;
  onConfig: (config: Config) => void;
  config: Config;
}) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        height: "100%",
        width: "100%",
      }}
    >
      <label
        style={{
          padding: "1rem",
        }}
      >
        <span>Confidence Threshold</span>
        <input
          type="number"
          max={1}
          min={0}
          defaultValue={config.confidenceThreshold}
          step={0.01}
          style={inputStyle}
          onChange={(e) => {
            onConfig(
              Object.assign({}, config, {
                confidenceThreshold: parseFloat(e.target.value),
              }),
            );
          }}
        />
      </label>

      <button
        style={{
          background: "none",
          border: "none",
          color: "white",
          fontSize: "1.5rem",
          cursor: "pointer",
          padding: "1rem",
        }}
        onClick={onClose}
      >
        <FaHome />
      </button>
    </div>
  );
}
