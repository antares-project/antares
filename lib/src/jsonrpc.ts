import { WS } from "./websocke.js";

type Version = "1.0" | "2.0";

type Request = {
	jsonrpc: Version;
	method: string;
	params: any[];
	id: number;
};

type ErrorValue = {
	code: number;
	message: string;
	data: any;
};

type ResultResponse = {
	jsonrpc: Version;
	result: any;
	id: number;
};

type ErrorResponse = {
	jsonrpc: Version;
	error: ErrorValue;
	id: number;
};

type Response = ResultResponse | ErrorResponse;
type Message = Request | Response;

export class Client<
	ClientToServer extends Record<keyof ClientToServer, (...args: any[]) => any>,
	ServerToClient extends Record<keyof ServerToClient, (...args: any[]) => void>
> {
	private socket: WS;
	private counter = 0;

	private pending = new Map<
		number,
		{
			resolve: (value: any) => void;
			reject: (reason?: any) => void;
		}
	>();
	private listeners = new Map<keyof ServerToClient, Set<ServerToClient[keyof ServerToClient]>>();

	public onOpen?: () => void;
	public onClose?: () => void;
	public onError?: () => void;

	public close: typeof WebSocket.prototype.close;

	constructor(url: string) {
		this.socket = new WS(url);
		this.close = this.socket.close.bind(this.socket);

		this.socket.onmessage = (event) => {
			try {
				const message: Message = JSON.parse(event.data);

				if ("method" in message) {
					const listeners = this.listeners.get(message.method as keyof ServerToClient);
					for (const listener of listeners ?? []) {
						const params =
							typeof message.params == "object"
								? Object.values(message.params)
								: message.params;
						listener(...params);
					}
				} else {
					const handler = this.pending.get(message.id);
					this.pending.delete(message.id);

					if ("error" in message) {
						handler?.reject(message.error);
					} else {
						handler?.resolve(message.result);
					}
				}
			} catch (error) {
				console.error("Failed to parse message:", event.data, error);
			}
		};

		this.socket.onopen = () => {
			this.onOpen?.();
		};

		this.socket.onclose = () => {
			for (const [_, handler] of this.pending) {
				handler.reject(new Error("Connection closed"));
			}

			this.pending.clear();
			this.onClose?.();
		};

		this.socket.onerror = () => {
			this.onError?.();
		};
	}

	call<K extends keyof ClientToServer>(method: K, ...params: Parameters<ClientToServer[K]>) {
		const id = this.counter++;
		const request = {
			jsonrpc: "2.0",
			method,
			params,
			id
		};
		this.socket.send(JSON.stringify(request));

		return new Promise<ReturnType<ClientToServer[K]>>((resolve, reject) => {
			this.pending.set(id, { resolve, reject });
		});
	}

	on<K extends keyof ServerToClient>(event: K, callback: ServerToClient[K]) {
		if (!this.listeners.has(event)) {
			this.listeners.set(event, new Set());
		}

		this.listeners.get(event)!.add(callback);

		return () => {
			this.listeners.get(event)?.delete(callback);
		};
	}
}
