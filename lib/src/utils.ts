import { codec } from "rfc4648";

const Z_BASE32 = {
	chars: "ybndrfg8ejkmcpqxot1uwisza345h769",
	bits: 5
};

export function stringToUint8Array(text: string) {
	return new TextEncoder().encode(text);
}

export function uint8ArrayToString(bytes: Uint8Array) {
	return new TextDecoder().decode(bytes);
}

export function uint8ArrayToZ32(bytes: Uint8Array) {
	return codec.stringify(bytes, Z_BASE32, { pad: false });
}

export function z32toUint8Array(text: string) {
	return codec.parse(text, Z_BASE32, { loose: true });
}
