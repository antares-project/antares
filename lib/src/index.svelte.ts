
import { Client as JsonRPCClient } from "./jsonrpc.js";
import { getInfo } from "./http.js";

type Session = {
    publicKey: Uint8Array,
    isAdmin: boolean,
    authToken: string,
    currentChannel?: Channel,
}

export class Client {
    private _url = $state("");
    private _rpc: JsonRPCClient<ClientToServerEvents, ServerToClientEvents>;
    private _session = $state<Session | undefined>();

    public onOpen?: () => void;
    public onClose?: () => void;
    public onError?: () => void;

    public onMessageReceived?: (message: Message) => void;

    public close: typeof JsonRPCClient.prototype.close;

    constructor(url: string) {
        this._rpc = new JsonRPCClient(`${url}/ws`);
        this._url = url;

        this.close = this._rpc.close.bind(this._rpc);

        this._rpc.onOpen = () => {
            this.onOpen?.();
        }
        this._rpc.onClose = () => {
            this._session = undefined;

            this.onClose?.();
        }
        this._rpc.onError = () => {
            this.onError?.();
        }

        this._rpc.on("messageReceived", (message) => {
            this.onMessageReceived?.(message);
        });
    }

    async getInfo() {
        return await getInfo(this._url);
    }

    async requestChallenge(publicKey: Uint8Array) {
        return await this._rpc.call("requestChallenge", publicKey.toHex());
    }

    async confirmChallenge(token: string, signature: Uint8Array) {
        return await this._rpc.call("confirmChallenge", token, signature.toHex());
    }

    async auth(token: string) {
        const payload = await this._rpc.call("auth", token);
        this._session = {
            publicKey: Uint8Array.fromHex(payload.public_key),
            isAdmin: payload.is_admin,
            authToken: token,
        };

        return payload;
    }

    async joinChannel(id: string) {
        if (!this._session) {
            throw new Error("Not authenticated");
        }

        const channel = await this._rpc.call("joinChannel", id);

        this._session.currentChannel = channel;

        return channel;
    }

    async loadMessages(beforeId?: string) {
        return await this._rpc.call("loadMessages", beforeId);
    }

    async sendMessage(message: string, attachments: string[]) {
        return await this._rpc.call("sendMessage", message, attachments);
    }

    async createChannel(name: string) {
        return await this._rpc.call("createChannel", name);
    }

    async deleteChannel(channelId: string) {
        return await this._rpc.call("deleteChannel", channelId);
    }

    async listChannels() {
        return await this._rpc.call("listChannels");
    }

    get url() {
        return this._url;
    }

    get isAuth() {
        return !!this._session;
    }

    get isAdmin() {
        return this._session?.isAdmin ?? false;
    }

    get publicKey() {
        return this._session?.publicKey;
    }

    get currentChannel() {
        return this._session?.currentChannel;
    }
}

export type ChannelType = "Text" | "Voice";

export interface Channel {
    id: string,
    name: string,
    type: ChannelType,
}

export interface Message {
    id: string,
    content: string,
    attachments: MessageAttachment[]
}

export interface MessageAttachment {
    id: string,
    name: string,
    mime_type: string,
    size: number,
    hash: string,
}

export interface ResponseAuthChallenge {
    token: string,
}

export interface ResponseConfirmAuthChallenge {
    token: string,
    payload: AuthenticatedPayload,
}

export interface AuthenticatedPayload {
    public_key: string,
    is_admin: boolean,
    exp: number,
}

export interface Channel {
    id: string,
    name: string,
}

interface ServerToClientEvents {
    messageReceived(message: Message): void,
    channelDeleted(channel: Channel): void
}

interface ClientToServerEvents {
    auth(token: string): AuthenticatedPayload,
    requestChallenge(publicKey: string): ResponseAuthChallenge,
    confirmChallenge(token: string, signature: string): ResponseConfirmAuthChallenge,

    joinChannel(channelId: string): Channel,
    sendMessage(message: string, attachments: string[]): void,
    loadMessages(beforeId?: string): Message[],

    createChannel(name: string): Channel,
    deleteChannel(channelId: string): Channel,
    listChannels(): Channel[]
}