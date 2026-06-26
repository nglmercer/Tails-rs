import { add, multiply } from "./math.ts";

export function sumAndProduct(a, b) {
    return add(a, b) + multiply(a, b);
}
