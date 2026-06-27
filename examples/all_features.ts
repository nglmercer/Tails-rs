// ============================================================
// Tails-rs — All Features Example
// Run:  cargo run -- examples/all_features.ts
// Watch: cargo run -- --watch examples/all_features.ts
// ============================================================

// --- Variables & Operators ---
console.log("--- Variables & Operators ---");

let x = 42;
const pi = 3.14;
var label = "hello";
let isActive = true;
let isNull = null;
let notDefined = undefined;

console.log(x, pi, label);

// Arithmetic
let sum = 10 + 5;
let diff = 10 - 3;
let prod = 4 * 5;
let div = 20 / 4;
let mod = 10 % 3;
let pow = 2 ** 10;
console.log("arith:", sum, diff, prod, div, mod, pow);

// Compound assignment
let a = 10;
a += 5;
a -= 3;
a *= 2;
a /= 4;
a %= 3;
console.log("compound:", a);

// Comparison
console.log("eq:", 5 == 5, 5 === 5, 5 != 3, 5 !== "5");

// Logical
console.log("logic:", true && false, true || false, !true);

// Increment / Decrement
let counter = 0;
counter++;
counter++;
counter++;
console.log("counter:", counter);

// Power
console.log("pow:", 2 ** 8);
console.log("bitwise NOT:", ~0);

// --- Control Flow ---
console.log("\n--- Control Flow ---");

// if / else
let score = 85;
if (score >= 90) {
    console.log("grade: A");
} else if (score >= 80) {
    console.log("grade: B");
} else {
    console.log("grade: C");
}

// for loop
let sumFor = 0;
for (let i = 1; i <= 5; i++) {
    sumFor += i;
}
console.log("for sum 1..5:", sumFor);

// while loop
let w = 0;
let sumWhile = 0;
while (w < 5) {
    sumWhile += w;
    w++;
}
console.log("while sum 0..4:", sumWhile);

// do-while
let dw = 0;
let sumDW = 0;
do {
    sumDW += dw;
    dw++;
} while (dw < 5);
console.log("do-while sum 0..4:", sumDW);

// switch
let day = 3;
let dayName = "";
switch (day) {
    case 1: dayName = "Mon"; break;
    case 2: dayName = "Tue"; break;
    case 3: dayName = "Wed"; break;
    case 4: dayName = "Thu"; break;
    case 5: dayName = "Fri"; break;
    default: dayName = "Weekend";
}
console.log("day:", dayName);

// ternary
let age = 20;
let status = age >= 18 ? "adult" : "minor";
console.log("ternary:", status);

// break and continue
let breakSum = 0;
for (let i = 0; i < 10; i++) {
    if (i === 5) break;
    breakSum += i;
}
console.log("break at 5:", breakSum);

let continueSum = 0;
for (let i = 0; i < 10; i++) {
    if (i % 2 === 0) continue;
    continueSum += i;
}
console.log("odd sum:", continueSum);

// for...in
let objKeys = { a: 1, b: 2, c: 3 };
let keyCount = 0;
for (let key in objKeys) {
    keyCount++;
}
console.log("for-in keys:", keyCount);

// --- Functions & Closures ---
console.log("\n--- Functions & Closures ---");

// Function declaration
function add(a, b) {
    return a + b;
}
console.log("add(3,7):", add(3, 7));

// Function expression
const multiply = function(a, b) {
    return a * b;
};
console.log("multiply(4,5):", multiply(4, 5));

// Arrow functions
const double = x => x * 2;
const square = (x) => x * x;
const addArrow = (a, b) => { return a + b; };
console.log("arrow:", double(7), square(5), addArrow(10, 20));

// Closures
function makeCounter() {
    let count = 0;
    return function() {
        count++;
        return count;
    };
}
const ctr = makeCounter();
console.log("closure:", ctr(), ctr(), ctr());

// Higher-order functions
function applyTwice(fn, val) {
    return fn(fn(val));
}
console.log("higher-order:", applyTwice(x => x + 10, 5));

// --- Classes (OOP) ---
console.log("\n--- Classes ---");

class Calculator {
    constructor(a, b) {
        this.a = a;
        this.b = b;
    }

    add() {
        return this.a + this.b;
    }

    mul() {
        return this.a * this.b;
    }

    static create(a, b) {
        return new Calculator(a, b);
    }
}

let calc = Calculator.create(3, 4);
console.log("class:", calc.add(), calc.mul());

// Inheritance
class Animal {
    constructor(name) {
        this.name = name;
    }
    speak() {
        return this.name + " makes a sound";
    }
}

class Dog extends Animal {
    constructor(name) {
        super(name);
    }
    speak() {
        return this.name + " barks";
    }
    fetch() {
        return this.name + " fetches the ball";
    }
}

let rex = new Dog("Rex");
console.log("inheritance:", rex.speak(), rex.fetch());
console.log("instanceof:", rex instanceof Dog, rex instanceof Animal);

// Getter and Setter
class Person {
    constructor(first, last) {
        this._first = first;
        this._last = last;
    }
    get fullName() {
        return this._first + " " + this._last;
    }
    set fullName(v) {
        let parts = v.split(" ");
        this._first = parts[0];
        this._last = parts[1];
    }
}

let p = new Person("John", "Doe");
console.log("getter:", p.fullName);
p.fullName = "Jane Smith";
console.log("setter:", p.fullName);

// Class expressions
var AnonClass = class {
    constructor() {
        this.msg = "anonymous";
    }
    greet() {
        return this.msg;
    }
};
let anonInst = new AnonClass();
console.log("class expr:", anonInst.greet());

// Multiple instances
class Counter {
    constructor(start) {
        this.count = start;
    }
    inc() {
        this.count++;
        return this.count;
    }
}
var ca = new Counter(0);
var cb = new Counter(10);
ca.inc(); ca.inc(); ca.inc();
cb.inc();
console.log("multi instance:", ca.count, cb.count);

// --- Promises & Async ---
console.log("\n--- Promises & Async ---");

// Promise.resolve
var pResult = 0;
Promise.resolve(99).then(function(v) { pResult = v; });
console.log("Promise.resolve:", pResult);

// Promise.reject + catch
var pErr = 0;
Promise.reject("err").catch(function(v) { pErr = v; });
console.log("Promise.catch:", pErr);

// Promise chaining
var chainResult = 0;
Promise.resolve(10)
    .then(function(v) { return v + 5; })
    .then(function(v) { chainResult = v; });
console.log("chain:", chainResult);

// Promise.all
var allResult = 0;
Promise.all([Promise.resolve(1), Promise.resolve(2), Promise.resolve(3)])
    .then(function(arr) { allResult = arr.length; });
console.log("Promise.all:", allResult);

// Promise.finally
var finallyResult = 0;
Promise.resolve(1)
    .then(function(v) { finallyResult += v; })
    .finally(function() { finallyResult += 10; });
console.log("finally:", finallyResult);

// Promise constructor
var ctorResult = 0;
var ctorPromise = new Promise(function(resolve) { resolve(42); });
ctorPromise.then(function(v) { ctorResult = v; });
console.log("constructor:", ctorResult);

// --- Timers ---
console.log("\n--- Timers ---");

var timerResult = "not fired";
setTimeout(function() {
    timerResult = "fired";
}, 0);
console.log("setTimeout registered:", timerResult);

var intervalResult = 0;
var id = setInterval(function() {
    intervalResult++;
}, 10);
clearInterval(id);
console.log("setInterval/clear:", intervalResult);

// --- JSON ---
console.log("\n--- JSON ---");

let jsonStr = JSON.stringify({ name: "Alice", age: 30 });
console.log("stringify:", jsonStr);

let parsed = JSON.parse('{"x": 10, "y": "hello"}');
console.log("parse:", parsed.x, parsed.y);

let roundTrip = JSON.parse(JSON.stringify({ a: 1, b: 2 }));
console.log("roundtrip:", roundTrip.a, roundTrip.b);

// --- Object Methods ---
console.log("\n--- Object Methods ---");

let obj = { a: 1, b: 2, c: 3 };
console.log("keys:", Object.keys(obj).length);
console.log("values:", Object.values(obj).length);
console.log("entries:", Object.entries(obj).length);

let target = { a: 1 };
Object.assign(target, { b: 2 }, { c: 3 });
console.log("assign:", target.a, target.b, target.c);

// --- Array Methods ---
console.log("\n--- Array Methods ---");

let arr = [1, 2, 3];

// push / pop / shift / unshift
let pushArr = [1, 2];
pushArr.push(3);
console.log("push:", pushArr.length);

let popArr = [1, 2, 3];
popArr.pop();
console.log("pop:", popArr.length);

let shiftArr = [1, 2, 3];
let shifted = shiftArr.shift();
console.log("shift:", shifted, shiftArr.length);

let unshiftArr = [2, 3];
unshiftArr.unshift(1);
console.log("unshift:", unshiftArr.length);

// map
let mapped = [1, 2, 3].map(function(x) { return x * 2; });
console.log("map:", mapped[0], mapped[1], mapped[2]);

// filter
let evens = [1, 2, 3, 4, 5, 6].filter(function(x) { return x % 2 === 0; });
console.log("filter:", evens.length);

// reduce
let total = [1, 2, 3, 4].reduce(function(acc, x) { return acc + x; }, 0);
console.log("reduce:", total);

// find / findIndex
let found = [10, 20, 30].find(function(x) { return x > 15; });
let foundIdx = [10, 20, 30].findIndex(function(x) { return x > 15; });
console.log("find:", found, foundIdx);

// some / every
let hasBig = [1, 2, 3].some(function(x) { return x > 2; });
let allPos = [1, 2, 3].every(function(x) { return x > 0; });
console.log("some/every:", hasBig, allPos);

// indexOf / includes
let idx = [10, 20, 30].indexOf(20);
let inc = [1, 2, 3].includes(2);
console.log("indexOf/includes:", idx, inc);

// join
let joined = ["a", "b", "c"].join("-");
console.log("join:", joined);

// reverse / sort
let reversed = [1, 2, 3].reverse();
let sorted = [3, 1, 4, 1, 5].sort();
console.log("reverse:", reversed[0], "sort:", sorted[0]);

// concat
let concat = [1, 2].concat([3, 4]);
console.log("concat:", concat.length);

// slice / splice
let sliced = [1, 2, 3, 4, 5].slice(1, 3);
console.log("slice:", sliced.length);

let spliced = [1, 2, 3, 4];
let removed = spliced.splice(1, 2);
console.log("splice:", removed.length, spliced.length);

// flat
let flat = [[1, 2], [3, 4]].flat();
console.log("flat:", flat.length);

// forEach
let feSum = 0;
[1, 2, 3].forEach(function(x) { feSum += x; });
console.log("forEach:", feSum);

// --- String Methods ---
console.log("\n--- String Methods ---");

console.log("charAt:", "hello".charAt(1));
console.log("charCodeAt:", "A".charCodeAt(0));
console.log("slice:", "hello".slice(1, 3));
console.log("substring:", "hello".substring(1, 4));
console.log("indexOf:", "hello world".indexOf("world"));
console.log("includes:", "hello".includes("ell"));
console.log("replace:", "hello world".replace("world", "JS"));
console.log("split:", "a,b,c".split(",").length);
console.log("trim:", "  hello  ".trim());
console.log("toLowerCase:", "HELLO".toLowerCase());
console.log("toUpperCase:", "hello".toUpperCase());
console.log("startsWith:", "hello".startsWith("hel"));
console.log("endsWith:", "hello".endsWith("llo"));
console.log("repeat:", "ha".repeat(3));
console.log("padStart:", "5".padStart(3, "0"));
console.log("padEnd:", "5".padEnd(3, "0"));

// --- Math ---
console.log("\n--- Math ---");

console.log("PI:", Math.PI);
console.log("E:", Math.E);
console.log("abs:", Math.abs(-5));
console.log("floor:", Math.floor(3.7));
console.log("ceil:", Math.ceil(3.2));
console.log("round:", Math.round(3.5));
console.log("min:", Math.min(3, 1, 4, 1, 5));
console.log("max:", Math.max(3, 1, 4, 1, 5));
console.log("pow:", Math.pow(2, 10));
console.log("sqrt:", Math.sqrt(16));
console.log("log:", Math.log(1));
console.log("sin:", Math.sin(0));
console.log("cos:", Math.cos(0));
console.log("tan:", Math.tan(0));

let rand = Math.random();
console.log("random 0-1:", rand >= 0.0 && rand <= 1.0);

// --- Error Handling ---
console.log("\n--- Error Handling ---");

try {
    let val = 10;
    console.log("try:", val);
} catch (e) {
    console.log("catch (should not reach)");
} finally {
    console.log("finally: done");
}

try {
    throw "custom error";
} catch (e) {
    console.log("caught:", e);
}

// --- Template Literals ---
console.log("\n--- Template Literals ---");

let name = "World";
let greeting = `Hello ${name}!`;
console.log(greeting);

let val = 10;
let msg = `The value is ${val * 2}`;
console.log(msg);

let a2 = "Hello";
let b2 = "World";
let combined = `${a2}, ${b2}!`;
console.log(combined);

// --- Global Functions ---
console.log("\n--- Global Functions ---");

console.log("parseInt:", parseInt("42"));
console.log("parseInt negative:", parseInt("-42"));
console.log("parseFloat:", parseFloat("3.14"));
console.log("isNaN NaN:", isNaN(NaN));
console.log("isNaN 42:", isNaN(42));
console.log("isFinite:", isFinite(42));

// Number methods
console.log("Number.parseInt:", Number.parseInt("42"));
console.log("Number.parseFloat:", Number.parseFloat("3.14"));
console.log("Number.isNaN:", Number.isNaN(NaN));
console.log("Number.isFinite:", Number.isFinite(42));

// --- Destructuring ---
console.log("\n--- Destructuring ---");

let [da, db, dc] = [10, 20, 30];
console.log("array destr:", da, db, dc);

let { x: ox, y: oy } = { x: 100, y: 200 };
console.log("obj destr:", ox, oy);

let [d1, , d3] = [1, 2, 3];
console.log("skip:", d1, d3);

// --- Spread ---
console.log("\n--- Spread ---");

let sa = [1, 2, 3];
let sb = [4, 5, 6];
let sc = [...sa, ...sb];
console.log("spread arr:", sc.length);

// --- Type Checks ---
console.log("\n--- Type Checks ---");

console.log("typeof 42:", typeof 42);
console.log("typeof hello:", typeof "hello");
console.log("typeof true:", typeof true);
console.log("typeof undefined:", typeof undefined);
console.log("typeof null:", typeof null);
console.log("void:", typeof (void 0));

// --- instanceof & in ---
console.log("\n--- instanceof & in ---");

class Foo {}
let foo = new Foo();
console.log("instanceof Foo:", foo instanceof Foo);

let inObj = { x: 1, y: 2 };
console.log("in operator:", "x" in inObj, "z" in inObj);

// --- Nullish & Identity ---
console.log("\n--- Identity ---");

console.log("null == null:", null == null);
console.log("null === null:", null === null);
console.log("undefined == undefined:", undefined == undefined);
console.log("NaN === NaN:", NaN === NaN);

console.log("\n========== ALL FEATURES DONE ==========");
