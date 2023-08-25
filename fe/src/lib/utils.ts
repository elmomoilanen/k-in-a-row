export async function sleep(wait: number) {
    return new Promise((resolve) => setTimeout(resolve, wait));
}
