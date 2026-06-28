// Test dynamic import functionality
const mod = await import("./sample_plugin.ts");
console.log("Module loaded:", mod);
console.log("Plugin name:", mod.default.metadata.name);
console.log("Plugin version:", mod.default.metadata.version);

// Call the plugin's hooks
if (mod.default.setup) {
  mod.default.setup();
}
if (mod.default.onLoad) {
  mod.default.onLoad();
}

console.log("Dynamic import test passed!");
