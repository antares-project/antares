export function stringToUint8Array(text: string) {
    return new TextEncoder().encode(text);
}

export function uint8ArrayToString(bytes: Uint8Array) {
    return new TextDecoder().decode(bytes);
}