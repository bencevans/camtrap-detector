import { FaHome } from "react-icons/fa";

const inputStyle = {
  width: "100%",
};

const labelStyle = {
  padding: "1rem",
};

export default function ConfigDialog({ onClose, onConfig, config }) {
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
      <label style={{ labelStyle }}>
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
                confidenceThreshold: parseFloat(e.target.value, 10),
              })
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
