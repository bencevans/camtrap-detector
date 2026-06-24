import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
// The Vite config helper and plugin factory types can be difficult to resolve in some environments.
// eslint-disable-next-line @typescript-eslint/no-unsafe-call
export default defineConfig({
  // eslint-disable-next-line @typescript-eslint/no-unsafe-call
  plugins: [react()],
});
