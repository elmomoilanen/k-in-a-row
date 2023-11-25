export async function sleep(wait: number) {
    return new Promise((resolve) => setTimeout(resolve, wait));
}

export function range(start: number, end: number) {
    return Array.from({ length: end - start }, (_, i) => start + i);
}
