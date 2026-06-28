// Test: dynamic import with await
const result = await import("./sample_plugin.ts");
console.log("Result:", result);
console.log("Result type:", typeof result);
