import { writable, type Writable } from "svelte/store";
import { parse } from "./utils";

export function useStorage<T>(key: string, defaultValue: T): Writable<T> {
    const storeKey = `useStorage:${key}`;

    const storedValue = localStorage.getItem(storeKey);
    const initialValue = parse<T>(storedValue) ?? defaultValue;

    const value = writable(initialValue);

    value.subscribe((s) => {
        localStorage.setItem(storeKey, JSON.stringify(s));
    });

    return value;
}