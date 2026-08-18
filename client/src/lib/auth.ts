import { uint8ArrayToZ32, z32toUint8Array } from "harmon-lib/utils";
import { getCookie, setCookie } from "./cookie";

export function login(publicKey: Uint8Array, privateKey: Uint8Array) {
	setCookie("publicKey", uint8ArrayToZ32(publicKey));
	setCookie("privateKey", uint8ArrayToZ32(privateKey));

	return { publicKey, privateKey };
}

export function useAuth() {
	const publicKeyHex = getCookie("publicKey");
	const privateKeyHex = getCookie("privateKey");

	if (!publicKeyHex || !privateKeyHex) {
		return undefined;
	}

	const publicKey = z32toUint8Array(publicKeyHex);
	const privateKey = z32toUint8Array(privateKeyHex);

	return { publicKey, privateKey };
}
