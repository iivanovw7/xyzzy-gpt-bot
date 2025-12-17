export {};

declare global {
	type Nullable<T> = null | T;
	type Maybe<T> = null | T | undefined;
}
