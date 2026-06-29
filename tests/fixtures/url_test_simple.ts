import assert from "assert";

console.log("Testing URL...");
const url = new URL("https://example.com/path?foo=bar&baz=qux#hash");
console.log("URL href:", url.href);
console.log("Type of href:", typeof url.href);
console.log("Expected value:", "https://example.com/path?foo=bar&baz=qux#hash");
console.log("Type of expected:", typeof "https://example.com/path?foo=bar&baz=qux#hash");

// Compare values manually
const actualStr = url.href;
const expectedStr = "https://example.com/path?foo=bar&baz=qux#hash";
console.log("Actual string:", actualStr);
console.log("Expected string:", expectedStr);
console.log("Are they equal?", actualStr === expectedStr);
console.log("Actual length:", actualStr.length);
console.log("Expected length:", expectedStr.length);

// Test basic assertion
assert.strictEqual(url.href, "https://example.com/path?foo=bar&baz=qux#hash");
console.log("First assertion passed");