import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  // eslint-disable-next-line @typescript-eslint/no-unsafe-call
  plugins: [react()],
  test: {
    environment: "jsdom",
    include: ["**/*.test.{ts,tsx}"],
  },
});
