import { parseErr, type Result } from "./error";

export function clamp(value: number, min: number, max: number) {
	return Math.min(Math.max(value, min), max);
}

export function debounce(callback: Function, wait = 1000) {
	let timeout: ReturnType<typeof setTimeout>;

	return (...args: any[]) => {
		clearTimeout(timeout);
		timeout = setTimeout(() => callback(...args), wait);
	};
}

export function parse<T>(input: any): T | undefined {
	const data = String(input);
	const [ok, value] = parseErr<T, any>(JSON.parse, data);
	if (ok) {
		return value;
	} else {
		return undefined;
	}
}