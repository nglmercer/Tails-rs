const classmap = new Map();
class MyClass {
  name = "hello";
  value = 42;
  greet() {
    return "hi";
  }
  add(a: number, b: number) {
    return a + b;
  }
}
const newclass = new MyClass();
classmap.set("newclass", newclass);
const getclass = classmap.get("newclass");
console.log(MyClass, newclass, getclass.add(1, 2));
