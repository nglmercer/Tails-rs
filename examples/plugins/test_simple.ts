// Simple test: just try dynamic import
const result = import("./sample_plugin.ts");
console.log("Import result type:", typeof result);
// The result should be a Promise
