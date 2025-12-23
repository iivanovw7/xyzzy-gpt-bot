/* eslint-disable @typescript-eslint/consistent-type-definitions */
/* eslint-disable @typescript-eslint/method-signature-style */
/* eslint-disable @typescript-eslint/no-explicit-any */
export {};

declare global {
	type Nullable<T> = null | T;
	type Maybe<T> = null | T | undefined;
	type Pixels = number;
	type Milliseconds = number;
	type Percent = number;
	type Voidable<T> = T | undefined | void;
	type Recordable<T = any> = Record<string, T>;
	type AnyFunction = (...arguments_: any[]) => any;
	type UnwrapPromise<T extends Promise<any>> = T extends Promise<infer Data> ? Data : never;
	type AsyncReturnType<T extends (...arguments_: any[]) => Promise<any>> = UnwrapPromise<ReturnType<T>>;
	type Optional<T extends object, K extends keyof T = keyof T> = Omit<T, K> & Partial<Pick<T, K>>;
	type RequiredFields<T, K extends keyof T> = Required<Pick<T, K>> & T;
	type Constructor<T = any> = new (...arguments_: any[]) => T;

	type AugmentedRequired<T extends object, K extends keyof T = keyof T> = Omit<T, K> & Required<Pick<T, K>>;
	/** Represents any object object. */
	type AnyObject<T = any> = {
		[field: string]: T;
	};

	/** Gets property type. */
	type PropertyType<TObject, TProperty extends keyof TObject> = TObject[TProperty];

	/** Represents type of object with partial and `nullable` fields. */
	type PartialAndNullable<T> = {
		[P in keyof T]?: null | T[P];
	};

	type ObjectOrNull<T = unknown> = Nullable<AnyObject<T>>;

	type OptionalObject<T = unknown> = Maybe<ObjectOrNull<T>>;

	/** Object containing promise. */
	type WithPromise<T = unknown> = {
		promise: Promise<T>;
	};
	type ValueOf<T> = T[keyof T];
	type ExtractType<T, U extends T> = T extends U ? T : never;
	type QueryResponse<T> = {
		data: T;
	};

	interface Window {
		Telegram?: Telegram;
	}
}

interface Telegram {
	WebApp?: TelegramWebApp;
}

interface TelegramWebApp {
	close(): void;
	CloudStorage?: TelegramCloudStorage;

	initData: string;
	ready(): void;

	version: string;
}

interface TelegramCloudStorage {
	getItem(key: string, callback: (error: unknown, value?: string) => void): void;
	removeItem(key: string, callback?: (error: unknown, success: boolean) => void): void;
	setItem(key: string, value: string, callback?: (error: unknown, success: boolean) => void): void;
}
