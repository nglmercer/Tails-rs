export default class Calculator {
    constructor() {
        this.value = 0;
    }

    add(n) {
        this.value = this.value + n;
        return this;
    }

    getResult() {
        return this.value;
    }
}
