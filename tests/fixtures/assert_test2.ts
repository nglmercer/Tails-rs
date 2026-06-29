console.log("Testing assert.strictEqual...");
import assert from "assert";

// Test with same string values
const a = "hello";
const b = "hello";
console.log("a:", a, "b:", b, "equal:", a === b);

assert.strictEqual(1, 1);
console.log("int test passed");

assert.strictEqual(a, b);
console.log("string test passed");