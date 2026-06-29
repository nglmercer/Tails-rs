import assert from "assert";

console.log("assert type:", typeof assert);
console.log("assert.strictEqual type:", typeof assert.strictEqual);

// Test with variables
const fn = assert.strictEqual;
console.log("fn type:", typeof fn);

try {
    fn.call(assert, 1, 1);
    console.log("fn.call test passed");
} catch(e) {
    console.log("fn.call test failed:", e);
}

try {
    assert.strictEqual(1, 1);
    console.log("direct call test passed");
} catch(e) {
    console.log("direct call test failed:", e);
}
