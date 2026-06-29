import assert from "assert";

// Test URL constructor
const url = new URL("https://example.com/path?foo=bar&baz=qux#hash");
assert.strictEqual(url.href, "https://example.com/path?foo=bar&baz=qux#hash");
assert.strictEqual(url.origin, "https://example.com");
assert.strictEqual(url.protocol, "https:");
assert.strictEqual(url.host, "example.com");
assert.strictEqual(url.hostname, "example.com");
assert.strictEqual(url.port, "");
assert.strictEqual(url.pathname, "/path");
assert.strictEqual(url.search, "?foo=bar&baz=qux");
assert.strictEqual(url.hash, "#hash");

// Test URLSearchParams
assert.strictEqual(url.searchParams.get("foo"), "bar");
assert.strictEqual(url.searchParams.get("baz"), "qux");
assert.strictEqual(url.searchParams.has("foo"), true);
assert.strictEqual(url.searchParams.has("nonexistent"), false);

// Test URLSearchParams getAll
const url2 = new URL("https://example.com?a=1&a=2&a=3");
const values = url2.searchParams.getAll("a");
assert.strictEqual(values.length, 3);
assert.strictEqual(values[0], "1");
assert.strictEqual(values[1], "2");
assert.strictEqual(values[2], "3");

// Test URLSearchParams set
const url3 = new URL("https://example.com?foo=bar");
url3.searchParams.set("foo", "baz");
assert.strictEqual(url3.searchParams.get("foo"), "baz");

// Test URLSearchParams append
const url4 = new URL("https://example.com?foo=bar");
url4.searchParams.append("foo", "baz");
assert.strictEqual(url4.searchParams.getAll("foo").length, 2);

// Test URLSearchParams delete
const url5 = new URL("https://example.com?foo=bar&baz=qux");
url5.searchParams.delete("foo");
assert.strictEqual(url5.searchParams.has("foo"), false);
assert.strictEqual(url5.searchParams.has("baz"), true);

// Test URLSearchParams size
const url6 = new URL("https://example.com?foo=bar&baz=qux");
assert.strictEqual(url6.searchParams.size, 2);

// Test URLSearchParams toString
const url7 = new URL("https://example.com?foo=bar&baz=qux");
assert.strictEqual(url7.searchParams.toString(), "foo=bar&baz=qux");

// Test URLSearchParams entries
const url8 = new URL("https://example.com?foo=bar&baz=qux");
const entries = url8.searchParams.entries();
assert.strictEqual(entries.length, 2);

// Test URLSearchParams keys
const url9 = new URL("https://example.com?foo=bar&baz=qux");
const keys = url9.searchParams.keys();
assert.strictEqual(keys.length, 2);
assert.strictEqual(keys[0], "foo");
assert.strictEqual(keys[1], "baz");

// Test URLSearchParams values
const url10 = new URL("https://example.com?foo=bar&baz=qux");
const values10 = url10.searchParams.values();
assert.strictEqual(values10.length, 2);
assert.strictEqual(values10[0], "bar");
assert.strictEqual(values10[1], "qux");

// Test URLSearchParams forEach
const url11 = new URL("https://example.com?foo=bar&baz=qux");
const collected: Array<{key: string, value: string}> = [];
url11.searchParams.forEach((value: string, key: string) => {
    collected.push({key, value});
});
assert.strictEqual(collected.length, 2);

// Test URL toString
const url12 = new URL("https://example.com/path?foo=bar#hash");
assert.strictEqual(url12.toString(), "https://example.com/path?foo=bar#hash");

// Test URL with port
const url13 = new URL("https://example.com:8080/path");
assert.strictEqual(url13.port, "8080");
assert.strictEqual(url13.host, "example.com:8080");

// Test URL with no path
const url14 = new URL("https://example.com");
assert.strictEqual(url14.pathname, "/");
assert.strictEqual(url14.search, "");

console.log("All URL tests passed!");