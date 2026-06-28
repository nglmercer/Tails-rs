// Sample plugin file that can be loaded via import()
// Usage: const plugin = await import("./sample_plugin.ts");

export default {
  metadata: { name: "sample", version: "1.0.0" },
  setup() {
    console.log("[sample] plugin setup");
  },
  onLoad() {
    console.log("[sample] plugin loaded");
  },
};
