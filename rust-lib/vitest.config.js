import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    // globals: true
    include: ["./__tests__/**/*.{test,spec}.?(c|m)[jt]s?(x)"],
    coverage: {
      reporter: ["text", "html"],
      reportsDirectory: "./report/coverage",
      exclude: [
        "**/*.tsx",
        "config.js",
        "config.*.js",
        "*.config.js",
        "index.js",
        "index.d.ts"
      ]
    }
  }
});