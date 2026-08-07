export type Result<T, E> = Ok<T> | Err<E>;
export type Ok<T> = [true, T];
export type Err<E> = [false, E];

export function ok<T>(value: T): Ok<T> {
    return [
        true,
        value
    ];
}

export function err<E>(value: E): Err<E> {
    return [
        false,
        value
    ];
}

export function parseErr<T, E>(fn: (...args: any[]) => T, ...args: any[]): Result<T, E>;
export function parseErr<T, E>(fn: (...args: any[]) => Promise<T>, ...args: any[]): Promise<Result<T, E>>;

export function parseErr<T, E>(fn: (...args: any[]) => T | Promise<T>, ...args: any[]): Result<T, E> | Promise<Result<T, E>> {
    try {
        const result = fn(...args);

        if (result instanceof Promise) {
            return result
                .then(value => ok<T>(value))
                .catch(error => err<E>(error));
        }

        return ok<T>(result);
    } catch (error) {
        return err<E>(error as E);
    }
}
