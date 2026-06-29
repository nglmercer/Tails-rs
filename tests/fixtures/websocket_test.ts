import assert from "assert";

// Test WebSocket constructor exists
assert.strictEqual(typeof WebSocket, "function");

// Test WebSocket prototype methods exist
const ws = new WebSocket("wss://echo.websocket.org");
assert.strictEqual(typeof ws.send, "function");
assert.strictEqual(typeof ws.close, "function");
assert.strictEqual(typeof ws.addEventListener, "function");
assert.strictEqual(typeof ws.removeEventListener, "function");

// Test WebSocket properties
assert.strictEqual(ws.url, "wss://echo.websocket.org");
assert.strictEqual(ws.readyState, 0); // CONNECTING
assert.strictEqual(ws.bufferedAmount, 0);
assert.strictEqual(ws.binaryType, "blob");
assert.strictEqual(ws.protocol, "");
assert.strictEqual(ws.extensions, "");

// Test WebSocket readyState constants
// These are typically static properties on the constructor
// For now, we'll just test the instance values

console.log("WebSocket API is available and has correct structure");

// Test that we can create a WebSocket instance
const ws2 = new WebSocket("wss://echo.websocket.org");
assert.strictEqual(ws2.url, "wss://echo.websocket.org");
assert.strictEqual(ws2.readyState, 0);

// Test event listener registration (just the API, not actual events)
const dummyCallback = () => {};
ws2.addEventListener("open", dummyCallback);
ws2.addEventListener("message", dummyCallback);
ws2.addEventListener("close", dummyCallback);
ws2.addEventListener("error", dummyCallback);

// Test that we can remove event listeners
ws2.removeEventListener("open", dummyCallback);

console.log("WebSocket tests passed!");