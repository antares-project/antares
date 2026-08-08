export { ResolvePolicy, SignedPacket } from "@synonymdev/pkarr";
import { Client as PkarrClient, ResolvePolicy, SignedPacket } from "@synonymdev/pkarr";

export interface AtypeRecordData {
    type: "A",
    address: string,
}

export interface AAAtypeRecordData {
    type: "AAA",
    address: string,
}

export interface HTTPSTypeRecordData {
    type: "HTTPS",
    priority: number,
    target: string,
}

export type RecordData = AtypeRecordData | AAAtypeRecordData | HTTPSTypeRecordData;

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

    async resolve(public_key_str: string, policy: ResolvePolicy): Promise<DnsEntry[] | null> {
        const value = await this.pkarrClient.resolve(public_key_str, policy);
        return (value?.records ?? []) as DnsEntry[];
    }
}