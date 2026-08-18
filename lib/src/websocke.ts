export type WSOptions = {
	autoReconnect?: boolean;
	autoReconnectDelay?: number;
};

export class WS {
	private socket: WebSocket;
	private manuallyClosed = false;
	private reconnectTimer?: ReturnType<typeof setTimeout>;
	private readonly targetUrl: string | URL;
	private readonly protocols?: string | string[];
	private readonly shouldReconnect: boolean;
	private readonly reconnectDelay: number;

	onopen: ((this: WebSocket, ev: Event) => any) | null = null;
	onerror: ((this: WebSocket, ev: Event) => any) | null = null;
	onclose: ((this: WebSocket, ev: CloseEvent) => any) | null = null;
	onmessage: ((this: WebSocket, ev: MessageEvent) => any) | null = null;

	constructor(targetUrl: string | URL, protocols?: string | string[], options?: WSOptions) {
		this.targetUrl = targetUrl;
		this.protocols = protocols;
		this.shouldReconnect = options?.autoReconnect ?? true;
		this.reconnectDelay = options?.autoReconnectDelay ?? 1000;

		this.socket = this.connect();
	}

	send(data: Parameters<WebSocket["send"]>[0]) {
		this.socket.send(data);
	}

	close(code?: number, reason?: string) {
		this.manuallyClosed = true;

		if (this.reconnectTimer) {
			clearTimeout(this.reconnectTimer);
			this.reconnectTimer = undefined;
		}

		this.socket.close(code, reason);
	}

	private connect() {
		const socket = new WebSocket(this.targetUrl, this.protocols);

		socket.onopen = (event) => {
			this.onopen?.call(socket, event);
		};

		socket.onmessage = (event) => {
			this.onmessage?.call(socket, event);
		};

		socket.onerror = (event) => {
			this.onerror?.call(socket, event);
		};

		socket.onclose = (event) => {
			this.onclose?.call(socket, event);

			if (!this.manuallyClosed && this.shouldReconnect) {
				this.reconnectTimer = setTimeout(() => {
					this.socket = this.connect();
				}, this.reconnectDelay);
			}
		};

		return socket;
	}
}
