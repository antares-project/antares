import { ed25519 } from "@noble/curves/ed25519.js";
import * as bip39 from "@scure/bip39";
import * as sha from "@noble/hashes/sha2.js";
import { wordlist as wlEn } from "@scure/bip39/wordlists/english.js";
import { wordlist as wlPt } from "@scure/bip39/wordlists/portuguese.js";
import { wordlist as wlSp } from "@scure/bip39/wordlists/spanish.js";

export const englishWordlist = wlEn;
export const portugueseWordlist = wlPt;
export const spanishWordlist = wlSp;

export function sha256(data: Uint8Array) {
	return sha.sha256(data);
}

export function sign(data: Uint8Array, privateKey: Uint8Array) {
	return ed25519.sign(data, privateKey);
}

export function verify(data: Uint8Array, signature: Uint8Array, publicKey: Uint8Array) {
	return ed25519.verify(signature, data, publicKey);
}

export function keygen(seed?: Uint8Array) {
	const { publicKey, secretKey: privateKey } = ed25519.keygen(seed);
	return { privateKey, publicKey };
}

export async function mnemonicToSeed(mnemonic: string[], passphrase?: string) {
	const seed = await bip39.mnemonicToSeed(mnemonic.join(" "), passphrase);
	return seed.slice(0, 32);
}

export function generateMnemonic(wordlist: string[] = englishWordlist, strength: number = 128) {
	return bip39.generateMnemonic(wordlist, strength).split(" ");
}
