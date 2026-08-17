export { ResolvePolicy, SignedPacket } from "@synonymdev/pkarr";
import { Client as PkarrClient, ResolvePolicy, SignedPacket } from "@synonymdev/pkarr";

export interface ARecordData {
    type: "A",
    address: string,
}

export interface AAAARecordData {
    type: "AAAA",
    address: string,
}

export interface HTTPSRecordData {
    type: "HTTPS",
    priority: number,
    target: string,
}

export interface TXTRecordData {
    type: "TXT",
    value: string,
}

export interface CNAMERecordData {
    type: "CNAME",
    target: string,
}

export type RecordData = ARecordData | AAAARecordData | HTTPSRecordData | TXTRecordData | CNAMERecordData;

export interface DnsEntry {
    name: string,
    ttl: number,
    rdata: RecordData,
}

export class DNSClient {
    private pkarrClient: PkarrClient

    constructor() {
        this.pkarrClient = new PkarrClient();
    }

    async publish(signed_packet: SignedPacket): Promise<number> {
        return this.pkarrClient.publish(signed_packet);
    }

    async resolve(public_key_str: string, policy: ResolvePolicy = ResolvePolicy.NetworkOnly): Promise<DnsEntry[] | null> {
        const value = await this.pkarrClient.resolve(public_key_str, policy);
        return (value?.records ?? []) as DnsEntry[];
    }

    async resolveUrl(url: string) {
        const haveProtocol = url.startsWith("https://") || url.startsWith("http://") || url.startsWith("wss://") || url.startsWith("ws://");
        const urlObject = new URL(haveProtocol ? url : `https://${url}`);

        const resolve = async (publicKey: string) => {
            const entries = await this.resolve(publicKey);

            for (const entry of entries ?? []) {
                switch (entry.rdata.type) {
                    case "HTTPS": {
                        return entry.rdata.target;
                    }
                    case "CNAME": {
                        return await resolve(entry.rdata.target);
                    }
                }
            }

            return null;
        }

        const host = await resolve(urlObject.host);

        if (!host) return null;

        urlObject.host = host;

        return urlObject.href.endsWith("/") ? urlObject.href.slice(0, -1) : urlObject.href;
    }
}