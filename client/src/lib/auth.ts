import { getCookie, setCookie } from './cookie';

export function login(publicKey: Uint8Array, privateKey: Uint8Array) {
    setCookie("publicKey", publicKey.toHex());
    setCookie("privateKey", privateKey.toHex());

    return { publicKey, privateKey }
}

export function useAuth() {
    const publicKeyHex = getCookie("publicKey");
    const privateKeyHex = getCookie("privateKey");

    if (!publicKeyHex || !privateKeyHex) {
        return undefined;
    }

    const publicKey = Uint8Array.fromHex(publicKeyHex);
    const privateKey = Uint8Array.fromHex(privateKeyHex);

    return { publicKey, privateKey };
}
