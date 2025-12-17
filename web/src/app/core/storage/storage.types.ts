import type { Theme } from "../../../types/styles.types";

export const StorageKey = {
	AccessToken: "accessToken",
	Theme: "theme",
} as const;

export type StorageKey = (typeof StorageKey)[keyof typeof StorageKey];

export type StorageValue = {
	[StorageKey.AccessToken]: Nullable<string>;
	[StorageKey.Theme]: Nullable<Theme>;
};

export const DEFAULT_STORAGE: Partial<StorageValue> = {
	[StorageKey.Theme]: null,
};
