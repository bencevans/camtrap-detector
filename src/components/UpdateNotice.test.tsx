import { StrictMode } from "react";
import { cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, expect, test, vi } from "vitest";
import { check, Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import UpdateNotice from "./UpdateNotice";

vi.mock("@tauri-apps/plugin-updater", () => ({ check: vi.fn() }));
vi.mock("@tauri-apps/plugin-process", () => ({ relaunch: vi.fn() }));

const downloadAndInstall = vi.fn<Update["downloadAndInstall"]>();
const close = vi.fn<Update["close"]>();
const onInstallingChange = vi.fn();
const update = { version: "1.0.5", downloadAndInstall, close } as Update;

beforeEach(() => {
  vi.clearAllMocks();
  vi.mocked(check).mockResolvedValue(update);
  vi.mocked(relaunch).mockResolvedValue(undefined);
  downloadAndInstall.mockResolvedValue(undefined);
  close.mockResolvedValue(undefined);
});

afterEach(cleanup);

test("checks on startup and waits for consent before installing", async () => {
  const user = userEvent.setup();
  render(<UpdateNotice canInstall onInstallingChange={onInstallingChange} />);
  const button = await screen.findByRole("button", { name: "Update and Restart" });
  expect(check).toHaveBeenCalledWith({ timeout: 10000 });
  expect(downloadAndInstall).not.toHaveBeenCalled();
  expect(relaunch).not.toHaveBeenCalled();

  await user.click(button);
  await waitFor(() => expect(relaunch).toHaveBeenCalledOnce());
  expect(downloadAndInstall).toHaveBeenCalledOnce();
  expect(downloadAndInstall.mock.invocationCallOrder[0])
    .toBeLessThan(vi.mocked(relaunch).mock.invocationCallOrder[0]);
  expect(onInstallingChange).toHaveBeenCalledWith(true);
});

test("allows postponing an update", async () => {
  const user = userEvent.setup();
  render(<UpdateNotice canInstall onInstallingChange={onInstallingChange} />);
  await user.click(await screen.findByRole("button", { name: "Later" }));
  expect(screen.queryByRole("status")).toBeNull();
  expect(downloadAndInstall).not.toHaveBeenCalled();
});

test("disables installation while a run or its results are open", async () => {
  const user = userEvent.setup();
  const view = render(<UpdateNotice canInstall={false} onInstallingChange={onInstallingChange} />);
  await user.click(await screen.findByRole("button", { name: "Update and Restart" }));
  expect(downloadAndInstall).not.toHaveBeenCalled();
  view.rerender(<UpdateNotice canInstall onInstallingChange={onInstallingChange} />);
  await user.click(screen.getByRole("button", { name: "Update and Restart" }));
  await waitFor(() => expect(downloadAndInstall).toHaveBeenCalledOnce());
});

test("leaves the app usable when offline or no update is available", async () => {
  const warning = vi.spyOn(console, "warn").mockImplementation(() => {});
  vi.mocked(check).mockRejectedValueOnce(new Error("offline"));
  const view = render(<UpdateNotice canInstall onInstallingChange={onInstallingChange} />);
  await waitFor(() => expect(warning).toHaveBeenCalledOnce());
  expect(screen.queryByRole("status")).toBeNull();
  view.unmount();
  vi.mocked(check).mockResolvedValueOnce(null);
  render(<UpdateNotice canInstall onInstallingChange={onInstallingChange} />);
  await waitFor(() => expect(check).toHaveBeenCalledTimes(2));
  expect(screen.queryByRole("status")).toBeNull();
  expect(relaunch).not.toHaveBeenCalled();
  warning.mockRestore();
});

test("does not restart after an installation failure and allows retry", async () => {
  const error = vi.spyOn(console, "error").mockImplementation(() => {});
  const user = userEvent.setup();
  downloadAndInstall.mockRejectedValueOnce(new Error("invalid signature"));
  render(<UpdateNotice canInstall onInstallingChange={onInstallingChange} />);
  await user.click(await screen.findByRole("button", { name: "Update and Restart" }));
  expect(await screen.findByRole("alert")).toBeTruthy();
  expect(relaunch).not.toHaveBeenCalled();
  expect(onInstallingChange).toHaveBeenLastCalledWith(false);
  await user.click(screen.getByRole("button", { name: "Update and Restart" }));
  await waitFor(() => expect(relaunch).toHaveBeenCalledOnce());
  error.mockRestore();
});

test("retries restart without reinstalling a completed update", async () => {
  const error = vi.spyOn(console, "error").mockImplementation(() => {});
  const user = userEvent.setup();
  vi.mocked(relaunch).mockRejectedValueOnce(new Error("restart failed"));
  render(<UpdateNotice canInstall onInstallingChange={onInstallingChange} />);
  await user.click(await screen.findByRole("button", { name: "Update and Restart" }));
  await user.click(await screen.findByRole("button", { name: "Restart" }));
  await waitFor(() => expect(relaunch).toHaveBeenCalledTimes(2));
  expect(downloadAndInstall).toHaveBeenCalledOnce();
  error.mockRestore();
});

test("closes update resources when a late startup check finishes after unmount", async () => {
  const resolves: ((value: Update | null) => void)[] = [];
  vi.mocked(check).mockImplementation(() => new Promise((done) => { resolves.push(done); }));
  const view = render(<StrictMode><UpdateNotice canInstall onInstallingChange={onInstallingChange} /></StrictMode>);
  view.unmount();
  resolves.forEach((resolve) => resolve(update));
  await waitFor(() => expect(close).toHaveBeenCalledTimes(2));
  expect(downloadAndInstall).not.toHaveBeenCalled();
});
