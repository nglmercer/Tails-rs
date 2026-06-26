import { fromA } from "./lib_a.ts";

export function fromB() {
    return fromA() + "B";
}
