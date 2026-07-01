let total = 0;
let passed = 0;
let failed = 0;

function test(name: string, fn: () => void) {
  total++;
  try {
    fn();
    passed++;
    console.log("  [PASS] " + name);
  } catch (e) {
    failed++;
    console.log("  [FAIL] " + name + ": " + e);
  }
}

function assert(condition: boolean, msg: string) {
  if (!condition) throw new Error(msg);
}

function assertEqual(a: any, b: any, msg: string) {
  if (a !== b) throw new Error(msg + ": expected " + JSON.stringify(b) + ", got " + JSON.stringify(a));
}

// ============================================================================
// Number.prototype methods
// ============================================================================

console.log("\n=== Number.prototype ===");

test("toFixed basic", () => {
  assertEqual((3.14159).toFixed(2), "3.14", "toFixed(2)");
});

test("toFixed 0 decimals", () => {
  assertEqual((3.7).toFixed(0), "4", "toFixed(0)");
});

test("toFixed many decimals", () => {
  assertEqual((0.1 + 0.2).toFixed(10), "0.3000000000", "toFixed(10)");
});

test("toFixed negative number", () => {
  assertEqual((-3.14).toFixed(1), "-3.1", "toFixed negative");
});

test("toFixed integer", () => {
  assertEqual((42).toFixed(2), "42.00", "toFixed integer");
});

test("toFixed NaN", () => {
  assertEqual(NaN.toFixed(2), "NaN", "toFixed NaN");
});

test("toFixed Infinity", () => {
  assertEqual(Infinity.toFixed(2), "Infinity", "toFixed Infinity");
});

test("toFixed -Infinity", () => {
  assertEqual((-Infinity).toFixed(2), "-Infinity", "toFixed -Infinity");
});

test("toString base 10", () => {
  assertEqual((255).toString(10), "255", "toString(10)");
});

test("toString base 16", () => {
  assertEqual((255).toString(16), "ff", "toString(16)");
});

test("toString base 2", () => {
  assertEqual((10).toString(2), "1010", "toString(2)");
});

test("toString base 8", () => {
  assertEqual((64).toString(8), "100", "toString(8)");
});

test("toString negative hex", () => {
  assertEqual((-255).toString(16), "-ff", "toString negative hex");
});

test("valueOf number", () => {
  assertEqual((42).valueOf(), 42, "valueOf");
});

test("toExponential", () => {
  assertEqual((1234.5).toExponential(2), "1.23e+3", "toExponential");
});

test("toExponential no args", () => {
  assertEqual((1234.5).toExponential(), "1.2345e+3", "toExponential no args");
});

test("toPrecision", () => {
  assertEqual((123.456).toPrecision(4), "123.5", "toPrecision");
});

test("Number.isInteger", () => {
  assertEqual(Number.isInteger(42), true, "isInteger(42)");
  assertEqual(Number.isInteger(3.14), false, "isInteger(3.14)");
  assertEqual(Number.isInteger(NaN), false, "isInteger(NaN)");
  assertEqual(Number.isInteger(Infinity), false, "isInteger(Infinity)");
});

test("Number.isSafeInteger", () => {
  assertEqual(Number.isSafeInteger(42), true, "isSafeInteger(42)");
  assertEqual(Number.isSafeInteger(2**53), false, "isSafeInteger(2^53)");
});

// ============================================================================
// Boolean.prototype methods
// ============================================================================

console.log("\n=== Boolean.prototype ===");

test("true.toString()", () => {
  assertEqual(true.toString(), "true", "true.toString()");
});

test("false.toString()", () => {
  assertEqual(false.toString(), "false", "false.toString()");
});

test("true.valueOf()", () => {
  assertEqual(true.valueOf(), true, "true.valueOf()");
});

test("false.valueOf()", () => {
  assertEqual(false.valueOf(), false, "false.valueOf()");
});

// ============================================================================
// String.prototype.matchAll
// ============================================================================

console.log("\n=== String.prototype.matchAll ===");

test("matchAll with global regex", () => {
  const str = "test1test2test3";
  const matches = str.matchAll(/test(\d)/g);
  assertEqual(matches.length, 3, "should find 3 matches");
  assertEqual(matches[0][0], "test1", "first match");
  assertEqual(matches[1][0], "test2", "second match");
  assertEqual(matches[2][0], "test3", "third match");
});

test("matchAll with no matches", () => {
  const matches = "hello".matchAll(/xyz/g);
  assertEqual(matches.length, 0, "no matches");
});

test("matchAll with capture groups", () => {
  const str = "aab";
  const matches = str.matchAll(/(a)b/g);
  assertEqual(matches.length, 1, "one match");
  assertEqual(matches[0][1], "a", "captured group");
});

// ============================================================================
// Math methods (verify existing ones still work)
// ============================================================================

console.log("\n=== Math methods ===");

test("Math.floor", () => {
  assertEqual(Math.floor(3.7), 3, "floor(3.7)");
  assertEqual(Math.floor(-3.7), -4, "floor(-3.7)");
});

test("Math.ceil", () => {
  assertEqual(Math.ceil(3.2), 4, "ceil(3.2)");
  assertEqual(Math.ceil(-3.2), -3, "ceil(-3.2)");
});

test("Math.round", () => {
  assertEqual(Math.round(3.5), 4, "round(3.5)");
  assertEqual(Math.round(3.4), 3, "round(3.4)");
});

test("Math.abs", () => {
  assertEqual(Math.abs(-5), 5, "abs(-5)");
  assertEqual(Math.abs(5), 5, "abs(5)");
});

test("Math.sqrt", () => {
  assertEqual(Math.sqrt(9), 3, "sqrt(9)");
});

test("Math.pow", () => {
  assertEqual(Math.pow(2, 3), 8, "pow(2,3)");
});

// ============================================================================
// Summary
// ============================================================================

console.log("\n==================================================");
console.log("Total: " + total + " tests, Passed: " + passed + ", Failed: " + failed);
console.log("==================================================");

if (failed > 0) {
  console.log("\nSome tests failed!");
} else {
  console.log("\nAll tests passed!");
}
