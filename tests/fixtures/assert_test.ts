import assert from "assert";

console.log("Testing assert.strictEqual...");
console.log("Assert object:", assert);
console.log("Assert.strictEqual:", assert.strictEqual);

// Test basic assertion
assert.strictEqual(1, 1);
console.log("Basic assertion passed");

// Test string assertion
assert.strictEqual("hello", "hello");
console.log("String assertion passed");

// Test with different values (should fail)
try {
    assert.strictEqual(1, 2);
    console.log("This should not print");
} catch (e) {
    console.log("Expected error:", e);
}

console.log("All assert tests passed!");