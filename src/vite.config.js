import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  // The plugin import's types can be difficult to resolve in some environments.
  // Disable the unsafe-call rule for this single invocation.
  // eslint-disable-next-line @typescript-eslint/no-unsafe-call
  plugins: [react()],
});
