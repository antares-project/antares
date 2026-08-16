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

export type RecordData = ARecordData | AAAARecordData | HTTPSRecordData | TXTRecordData;

export interface DnsEntry {
    name: string,
    ttl: number,
    rdata: RecordData,
}

export interface DnsServer {
    a: ARecordData,
    aaaa: AAAARecordData,
    https: HTTPSRecordData,
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

    async resolveProfileServer(public_key_str: string, policy: ResolvePolicy = ResolvePolicy.NetworkOnly): Promise<string | null> {
        const entry = await this.resolve(public_key_str, policy);

        if (!entry) return null;

        for (const record of entry) {
            if (record.name === "harmon" && record.rdata.type === "TXT") {
                return record.rdata.value;
            }
        }

        return null;
    }

    async resolveServer(public_key_str: string, policy: ResolvePolicy = ResolvePolicy.NetworkOnly): Promise<DnsServer | null> {
        const entry = await this.resolve(public_key_str, policy);
        if (!entry) return null;

        let a: ARecordData | undefined;
        let aaaa: AAAARecordData | undefined;
        let https: HTTPSRecordData | undefined;

        for (const record of entry) {
            if (record.rdata.type === "A" && record.name === public_key_str) a = record.rdata;
            if (record.rdata.type === "AAAA" && record.name === public_key_str) aaaa = record.rdata;
            if (record.rdata.type === "HTTPS" && record.name === public_key_str) https = record.rdata;
        }

        if (!a || !aaaa || !https) return null;

        return { a, aaaa, https };
    }
}