import { useEffect, useState } from "react";
import { check, Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

export default function UpdateNotice({
  canInstall,
  onInstallingChange,
}: {
  canInstall: boolean;
  onInstallingChange: (installing: boolean) => void;
}) {
  const [update, setUpdate] = useState<Update | null>(null);
  const [installing, setInstalling] = useState(false);
  const [installed, setInstalled] = useState(false);
  const [dismissed, setDismissed] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    let pendingUpdate: Update | null = null;
    check({ timeout: 10000 })
      .then((available) => {
        pendingUpdate = available;
        if (cancelled) {
          void available?.close().catch(console.error);
        } else {
          setUpdate(available);
        }
      })
      .catch((e: unknown) => console.warn("Could not check for updates:", e));

    return () => {
      cancelled = true;
      void pendingUpdate?.close().catch(console.error);
    };
  }, []);

  const installUpdate = async () => {
    if (!update || !canInstall || installing) return;
    setInstalling(true);
    onInstallingChange(true);
    setError(null);
    try {
      if (!installed) {
        await update.downloadAndInstall();
        setInstalled(true);
      }
      await relaunch();
    } catch (e) {
      console.error("Could not update the app:", e);
      setError("The update could not finish. Please try again.");
      setInstalling(false);
      onInstallingChange(false);
    }
  };

  if (!update || dismissed) return null;

  return (
    <div className="update-notice" role="status" aria-live="polite">
      <p>CamTrap Detector {update.version} is available.</p>
      {installing ? <p>Updating… The app will restart when ready.</p> : null}
      {!canInstall ? <p>Finish exporting, then select New Run to update.</p> : null}
      {error ? <p role="alert">{error}</p> : null}
      <button
        disabled={!canInstall || installing}
        onClick={() => { void installUpdate(); }}
      >
        {installed ? "Restart" : "Update and Restart"}
      </button>
      <button disabled={installing} onClick={() => setDismissed(true)}>
        Later
      </button>
    </div>
  );
}
