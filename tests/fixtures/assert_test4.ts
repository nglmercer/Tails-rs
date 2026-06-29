import assert from "assert";

// Test with strings directly
try {
    assert.strictEqual("hello", "hello");
    console.log("string test passed");
} catch(e) {
    console.log("string test failed:", e);
}

// Test with numbers
try {
    assert.strictEqual(1, 1);
    console.log("int test passed");
} catch(e) {
    console.log("int test failed:", e);
}
