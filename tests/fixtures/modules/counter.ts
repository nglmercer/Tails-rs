let count = 0;

export function increment() {
    count = count + 1;
    return count;
}

export function getCount() {
    return count;
}
