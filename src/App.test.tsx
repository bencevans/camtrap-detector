import { cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, expect, test, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import App from "./App";
import { check, Update } from "@tauri-apps/plugin-updater";

vi.mock("@tauri-apps/plugin-updater", () => ({
  check: vi.fn().mockResolvedValue(null),
}));
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));
vi.mock("@tauri-apps/api/window", async () => ({
  ...(await vi.importActual<typeof import("@tauri-apps/api/window")>(
    "@tauri-apps/api/window",
  )),
  getCurrentWindow: () => ({ setSize: vi.fn().mockResolvedValue(undefined) }),
}));
vi.mock("@tauri-apps/api/webview", () => ({
  getCurrentWebview: () => ({
    onDragDropEvent: vi.fn().mockResolvedValue(() => {}),
  }),
}));

beforeEach(() => {
  vi.clearAllMocks();
  vi.mocked(check).mockResolvedValue(null);
  vi.mocked(invoke).mockResolvedValue(undefined);
  vi.mocked(open).mockResolvedValue("/camera-trap-images");
});

afterEach(cleanup);

test.each([true, false])(
  "starts detection after selecting a folder with Include Subfolders = %s",
  async (recursive) => {
    const user = userEvent.setup();
    render(<App />);

    if (!recursive) {
      await user.click(screen.getByRole("checkbox", { name: "Include Subfolders" }));
    }
    await user.click(screen.getByRole("button", { name: "Select Folder" }));

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith("process", {
        path: "/camera-trap-images",
        confidenceThreshold: 0.1,
        recursive,
      });
    });
    expect(vi.mocked(invoke).mock.calls.filter(([command]) => command === "process"))
      .toHaveLength(1);
  },
);

test("does not start detection before a folder is selected or when selection is cancelled", async () => {
  const user = userEvent.setup();
  vi.mocked(open).mockResolvedValue(null);
  render(<App />);

  expect(invoke).not.toHaveBeenCalledWith("process", expect.anything());
  await user.click(screen.getByRole("checkbox", { name: "Include Subfolders" }));
  await user.click(screen.getByRole("button", { name: "Select Folder" }));

  expect(open).toHaveBeenCalledOnce();
  expect(invoke).not.toHaveBeenCalledWith("process", expect.anything());
});


test("does not allow an update to interrupt a selected detection run", async () => {
  const user = userEvent.setup();
  const downloadAndInstall = vi.fn();
  vi.mocked(check).mockResolvedValue({ version: "1.0.5", downloadAndInstall, close: vi.fn().mockResolvedValue(undefined) } as unknown as Update);
  render(<App />);
  const updateButton = await screen.findByRole("button", { name: "Update and Restart" });
  await user.click(screen.getByRole("button", { name: "Select Folder" }));
  await user.click(updateButton);
  expect(downloadAndInstall).not.toHaveBeenCalled();
});

test("prevents starting a detection run while an update is installing", async () => {
  const user = userEvent.setup();
  vi.mocked(check).mockResolvedValue({ version: "1.0.5", downloadAndInstall: vi.fn(() => new Promise<void>(() => {})), close: vi.fn().mockResolvedValue(undefined) } as unknown as Update);
  render(<App />);
  await user.click(await screen.findByRole("button", { name: "Update and Restart" }));
  expect(screen.queryByRole("button", { name: "Select Folder" })).toBeNull();
  expect(invoke).not.toHaveBeenCalledWith("process", expect.anything());
});
