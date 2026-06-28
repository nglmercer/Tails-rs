// Test static import first
import plugin from "./sample_plugin.ts";
console.log("Static import:", plugin);
console.log("Plugin name:", plugin.metadata.name);
