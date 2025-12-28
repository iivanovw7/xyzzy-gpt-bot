export const StorageKey = {
	AccessToken: "accessToken",
} as const;

export type StorageKey = (typeof StorageKey)[keyof typeof StorageKey];

export type StorageValue = {
	[StorageKey.AccessToken]: Nullable<string>;
};

export const DEFAULT_STORAGE: Partial<StorageValue> = {};
